use std::sync::Arc;

use teloxide::{dispatching::{HandlerExt, UpdateFilterExt}, prelude::*, utils::command::BotCommands};

use crate::{check_state::check_state, exceptions::TapeAppError, service::{TapeResponse, TapeState, TapeStatusInfo}, storage::{Subscribers, Tapes, UserSubscription, ValidUsers}};


pub async fn run_telegram_bot<T>(
    tape_service: Arc<T>,
    tapes: Tapes,
    valid_users: ValidUsers,
    subscribers: Subscribers,
) -> ()
where T: TapeStatusInfo + Send + Sync + 'static
{
    let bot = create_bot();

    check_state::<T>(
        bot.clone(),
        tape_service.clone(),
        tapes.clone(),
        subscribers.clone()
    ).await;

    Dispatcher::builder(
        bot,
        Update::filter_message().filter_command::<Command>().endpoint(answer::<T>)
    ).dependencies(dptree::deps![tape_service, tapes,  subscribers, valid_users])
        .build()
        .dispatch()
        .await;
}

fn create_bot() -> Bot {
    log::info!("Creating bot");
    Bot::from_env()
}

#[derive(BotCommands, Clone)]
#[command(rename_rule="kebab-case", description="These commands are supported:")]
enum Command {
    #[command(description="Show info")]
    Help,
    #[command(description="Subscribe tape events")]
    Subscribe,
    #[command(description="Unsubscribe tape events")]
    Unsubscribe,
    #[command(description="Get status taper")]
    Status
}

async fn answer<T>(
    bot: Bot,
    tape_service: Arc<T>,
    tapes: Tapes,
    subscribers: Subscribers,
    valid_users: ValidUsers,
    message: Message,
    cmd: Command
) -> ResponseResult<()>
where T: TapeStatusInfo
{
    let user_id = message.clone().from.unwrap().id.0;
    let chat_id = message.chat.id;

    if !valid_users.contains(&user_id) {
        bot.send_message(chat_id,
            format!("You don't have access to the bot. Your user telegram id {}", &user_id)).await?;
        return Ok(());
    }

    match cmd {
        Command::Help => {
            bot.send_message(chat_id, Command::descriptions().to_string()).await?;
        },
        Command::Subscribe => {
            subscribers.lock().await
                .insert(user_id, UserSubscription {
                    telegram_id: user_id,
                    chat_id: chat_id.0
                });
            bot.send_message(chat_id, "Subscribe on tape events").await?;
        },
        Command::Unsubscribe => {
            subscribers.lock().await.remove(&user_id);
            bot.send_message(message.chat.id, "Unsubscribe on tape events").await?;
        },
        Command::Status => {
            let mut inner_tapes = tapes.lock().await;
            for tape in inner_tapes.values_mut().into_iter() {
            let service_response = tape_service.get_tape_status(tape);
                let response = form_status_of_tape(&service_response, tape.get_path().to_str().expect("valid path"));
                bot.send_message(message.chat.id, response).await?;
            }
        }
    };
    Ok(())
}

pub fn form_status_of_tape
(
    service_response: &Result<TapeResponse, TapeAppError>,
    tape_path: &str
) -> String
{
    let form_body = match service_response {
        Ok(command_response) => {
            format!(
"Состояние ленты:
{}

Ответ команды:
{}",
                tape_state_rep(command_response.get_tape_status()),
                command_response.get_external_response()
            )
        },
        Err(error) => {
            match error {
                TapeAppError::ExecuteTapeAppError(app_error) => {
                    format!(
"Ошибка выполнения ПРОГРАММЫ.
Произошла следующая ошибка:

{}",
                        app_error
                    )
                }
                TapeAppError::ExecuteCommandTapeError(command_error) => {
                    format!(
"Ошибка выполнения КОМАНДЫ.
Произошла следующая ошибка:

{}",
                        command_error
                    )

                }
            }
        }

    };

    format!(
"Состояние ленты по следующему пути:
{}

Состояние:

{}",
        tape_path,
        form_body
    )
}


fn tape_state_rep(tape_state: &TapeState) -> String {
    match tape_state {
//        TapeState::Waiting => "Ожидание".to_string(),
        TapeState::Busy => "Девайс занят".to_string(),
        TapeState::Unknown => "Неизвестно".to_string()

    }
}

