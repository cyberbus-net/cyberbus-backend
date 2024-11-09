use actix_web::web::Json;
use lemmy_api_common::{
    context::LemmyContext,
    utils::send_verification_email,
    person::{SendVerifyEmail, SendVerifyEmailResponse},
};
use lemmy_db_views::structs::LocalUserView;
use lemmy_utils::error::{LemmyError, LemmyErrorType, LemmyResult};
use activitypub_federation::config::Data;

#[tracing::instrument(skip(context))]
pub async fn send_verify_email(
    data: Json<SendVerifyEmail>,
    context: Data<LemmyContext>,
) -> LemmyResult<Json<SendVerifyEmailResponse>> {
    let email = data.email.clone();
    
    // Check if email exists
    let local_user_view = LocalUserView::find_by_email(
        &mut context.pool(),
        &email
    )
    .await?
    .ok_or_else(|| LemmyError::from(LemmyErrorType::EmailNotFound))?;

    // Check if user's email is already verified
    if local_user_view.local_user.email_verified {
        return Err(LemmyError::from(LemmyErrorType::EmailAlreadyVerified));
    }

    // Send verification email
    send_verification_email(
        &local_user_view,
        &email,
        &mut context.pool(),
        context.settings(),
    )
    .await?;

    Ok(Json(SendVerifyEmailResponse {
        email_sent: true,
    }))
} 
