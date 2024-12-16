use std::{sync::Arc, time::Duration};

use teloxide::{prelude::Requester, types::ChatId, Bot};

use crate::{service:: TapeStatusInfo, storage::{Subscribers, Tapes}, telegram::form_status_of_tape};


const DURATION_TIME_MIN: u64 = 5*60;


pub async fn check_state<T>(
    bot: Bot,
    tape_service: Arc<T>,
    tapes: Tapes,
    subscribers: Subscribers,

)
where T: TapeStatusInfo + Send + Sync + 'static
{
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(DURATION_TIME_MIN)).await;

            for tape in tapes.lock().await.values_mut() {
                let previous_state = tape.get_state().clone();

                let tape_response = tape_service.get_tape_status(tape);

                match tape_response {
                    Ok(ref ok_tape_response) => {
                        if ok_tape_response.get_tape_status() == &previous_state {
                            continue;
                        }
                    },
                    Err(_) => { continue; }
                }

                let response = form_status_of_tape(&tape_response, tape.get_path().to_str().expect("Valid path"));

                let response = format!("Состояние ленты изменено\n{}", response);

                for subscriber in subscribers.lock().await.values() {
                    let chat_id: ChatId = ChatId(subscriber.chat_id);
                    if let Err(_) = bot.send_message(chat_id, response.clone()).await {
                        continue;
                    }

                }
            }
        }
    });
}
