
use twilio::Client;
use twilio::OutboundMessage;


#[tokio::main]
async fn main() {
    let to = "ENTER-TO-NUMBER-HERE";
    let from = "ENTER-FROM-NUMBER-FROM-TWILIO-HERE";
    let body = "ENTERT-TEXT-HERE";
    let app_id = "ENTER-APP-ID-HERE";
    let auth_token = "ENTER-AUTH-TOKEN-HERE";
    let client = Client::new(app_id, auth_token);
    let msg = OutboundMessage::new(from, to, body);
    client.send_message(msg).await; 
    }
