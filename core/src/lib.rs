use std::{env, io::Write, path::PathBuf};

use agent_client_protocol::{
    AcpAgent, Agent, Client, ConnectionTo,
    schema::{
        ProtocolVersion,
        v1::{
            ContentBlock, InitializeRequest, NewSessionRequest, PromptRequest,
            RequestPermissionOutcome, RequestPermissionRequest, RequestPermissionResponse,
            SelectedPermissionOutcome, SessionNotification, SessionUpdate, TextContent,
        },
    },
};

pub struct FlowAgentCore {}

impl FlowAgentCore {
    pub async fn run() {
        let agent = AcpAgent::from_args([
            "cmd",
            "/C",
            r"C:\Users\Amir Ali\AppData\Roaming\npm\opencode.cmd",
            "acp",
        ])
        .unwrap();

        Client
            .builder()
            .on_receive_notification(
                async move |notification: SessionNotification, _cx| {
                    match notification.update {
                        SessionUpdate::AgentMessageChunk(chunk) => {
                            // This is the actual reply text — stream it without newlines
                            if let ContentBlock::Text(text) = chunk.content {
                                print!("{}", text.text);
                                std::io::stdout().flush().ok();
                            }
                        }
                        SessionUpdate::AgentThoughtChunk(_) => {
                            // Reasoning/thinking tokens — usually noise for end users, skip or dim them
                            // eprint!("."); // optional: show progress without content
                        }
                        SessionUpdate::ToolCallUpdate(tool_call) => {
                            eprintln!("\n🔧 tool: {:?}", tool_call);
                        }
                        SessionUpdate::AvailableCommandsUpdate(_) => {
                            // fires once at session start, usually not interesting to print
                        }
                        SessionUpdate::UsageUpdate(usage) => {
                            eprintln!("\n(tokens used: {}/{})", usage.used, usage.size);
                        }
                        other => {
                            eprintln!("\n[unhandled update]: {:?}", other);
                        }
                    }
                    Ok(())
                },
                agent_client_protocol::on_receive_notification!(),
            )
            .on_receive_request(
                async move |request: RequestPermissionRequest, responder, _connection| {
                    eprintln!("✅ Auto-approving permission request: {request:?}");
                    let option_id = request.options.first().map(|opt| opt.option_id.clone());
                    if let Some(id) = option_id {
                        responder.respond(RequestPermissionResponse::new(
                            RequestPermissionOutcome::Selected(SelectedPermissionOutcome::new(id)),
                        ))
                    } else {
                        eprintln!("⚠️ No options provided in permission request, cancelling");
                        responder.respond(RequestPermissionResponse::new(
                            RequestPermissionOutcome::Cancelled,
                        ))
                    }
                },
                agent_client_protocol::on_receive_request!(),
            )
            .connect_with(agent, |connection: ConnectionTo<Agent>| async move {
                println!("Initializing agent...");
                let init_response = connection
                    .send_request(InitializeRequest::new(ProtocolVersion::V1))
                    .block_task()
                    .await
                    .unwrap();

                println!("✓ Agent initialized: {:?}", init_response.agent_info);

                println!("📝 Creating new session...");

                let new_session_response = connection
                    .send_request(NewSessionRequest::new(
                        env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
                    ))
                    .block_task()
                    .await
                    .unwrap();

                let session_id = new_session_response.session_id;
                println!("Session created");

                println!("Sending prompt: \"Hi\"");
                let prompt_response = connection
                    .send_request(PromptRequest::new(
                        session_id.clone(),
                        vec![ContentBlock::Text(TextContent::new("Hi"))],
                    ))
                    .block_task()
                    .await
                    .unwrap();

                println!(); // flush the streamed line
                eprintln!(
                    "✅ Agent completed! Stop reason: {:?}",
                    prompt_response.stop_reason
                );

                Ok(())
            })
            .await
            .unwrap();
    }
}
