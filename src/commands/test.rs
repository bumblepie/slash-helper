use serenity::{
    async_trait,
    client::Context,
    model::{
        channel::PartialChannel,
        guild::{PartialMember, Role},
        interactions::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        prelude::User,
    },
};
use slash_helper::{parsable::Mentionable, ApplicationCommandInteractionHandler, InvocationError};
use slash_helper_macros::Command;

/// A test command for checking the macros are working
#[allow(dead_code)]
#[derive(Command, Debug)]
#[name = "test"]
pub struct TestCommand {
    /// a string option
    string_opt: String,
    /// a non-required string option
    maybe_string_opt: Option<String>,
    /// an int option
    int_opt: i64,
    /// a non-required int option
    maybe_int_opt: Option<i64>,
    /// a bool option
    bool_opt: bool,
    /// a non-required bool option
    maybe_bool_opt: Option<bool>,
    /// a number option
    num_opt: f64,
    /// a non-required num option
    maybe_num_opt: Option<f64>,
    /// a user option
    user_opt: (User, Option<PartialMember>),
    /// a non-required user option
    maybe_user_opt: Option<(User, Option<PartialMember>)>,
    /// a role option
    role_opt: Role,
    /// a non-required role option
    maybe_role_opt: Option<Role>,
    /// a mentionable option
    mentionable_opt: Mentionable,
    /// a non-required role option
    maybe_mentionable_opt: Option<Mentionable>,
    /// a channel option
    channel_opt: PartialChannel,
    /// a non-required role option
    maybe_channel_opt: Option<PartialChannel>,
}

#[async_trait]
impl ApplicationCommandInteractionHandler for TestCommand {
    async fn invoke(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> Result<(), InvocationError> {
        println!("{:?}", self);
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|data| {
                        data.content("Test command complete, see logs for received command details")
                    })
            })
            .await
            .expect("Failed to send test command response");
        Ok(())
    }
}
