//! SMS CLI commands

use crate::{cli::OutputFormat, output::format_output};
use anyhow::Result;
use clap::Subcommand;
use huawei_dongle_api::{
    models::{sms::SmsListRequest, SmsBoxType, SmsSortType},
    Client,
};

#[derive(Subcommand)]
pub enum SmsCommands {
    /// Get SMS message count
    Count,

    /// List SMS messages
    List {
        /// Page index (starting from 1)
        #[arg(long, default_value = "1")]
        page: u32,

        /// Number of messages to retrieve per page
        #[arg(long, default_value = "20")]
        count: u32,

        /// Show only unread messages
        #[arg(long)]
        unread: bool,

        /// Show message content in table format
        #[arg(long)]
        show_content: bool,
    },

    /// Delete SMS message by ID
    Delete {
        /// Message ID to delete
        message_id: String,

        /// Skip confirmation prompt
        #[arg(long)]
        yes: bool,
    },

    /// Mark SMS message as read
    MarkRead {
        /// Message ID to mark as read
        message_id: String,
    },
}

impl SmsCommands {
    pub async fn execute(&self, client: &Client, format: &OutputFormat) -> Result<()> {
        match self {
            SmsCommands::Count => {
                let count = client.sms().count().await?;

                match format {
                    OutputFormat::Table => {
                        println!("SMS Message Count:");
                        println!();
                        println!("Local Storage:");
                        println!("  Unread: {}", count.local_unread);
                        println!("  Inbox:  {}", count.local_inbox);
                        println!("  Outbox: {}", count.local_outbox);
                        println!("  Draft:  {}", count.local_draft);
                        println!();
                        println!("SIM Storage:");
                        println!("  Unread: {}", count.sim_unread);
                        println!("  Inbox:  {}", count.sim_inbox);
                        println!("  Outbox: {}", count.sim_outbox);
                        println!("  Draft:  {}", count.sim_draft);
                        println!();
                        println!("Total Unread: {}", count.total_unread().unwrap_or(0));
                        println!("Total Inbox:  {}", count.total_inbox().unwrap_or(0));
                        if count.has_new_messages() {
                            println!("New Messages: Yes");
                        }
                    }
                    _ => {
                        format_output(&count, format)?;
                    }
                }
            }

            SmsCommands::List {
                page,
                count,
                unread,
                show_content,
            } => {
                let request = SmsListRequest::new(
                    *page,
                    *count,
                    SmsBoxType::LocalInbox,
                    SmsSortType::ByTime,
                    false,
                    *unread, // unread preferred if filtering for unread
                );

                let response = client.sms().list(&request).await?;
                let mut messages = response.messages.messages;

                // Filter for unread if requested
                if *unread {
                    messages.retain(|msg| msg.is_unread());
                }

                if messages.is_empty() {
                    println!("No messages found");
                    return Ok(());
                }

                match format {
                    OutputFormat::Table => {
                        println!("SMS Messages ({} found):", messages.len());
                        println!();

                        for message in &messages {
                            println!(
                                "ID: {} | From: {} | Date: {} | Status: {}",
                                message.id(),
                                message.phone_number(),
                                message.date_str(),
                                if message.is_unread() {
                                    "Unread"
                                } else {
                                    "Read"
                                }
                            );

                            if *show_content {
                                println!("Content: {}", message.text());
                            }
                            println!();
                        }
                    }
                    _ => {
                        format_output(&messages, format)?;
                    }
                }
            }

            SmsCommands::Delete { message_id, yes } => {
                if !yes {
                    println!(
                        "Are you sure you want to delete SMS message {}? [y/N]",
                        message_id
                    );
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    if !input.trim().to_lowercase().starts_with('y') {
                        println!("Cancelled");
                        return Ok(());
                    }
                }

                client.sms().delete(message_id).await?;
                println!("SMS message {} deleted successfully", message_id);
            }

            SmsCommands::MarkRead { message_id } => {
                client.sms().mark_read(message_id).await?;
                println!("SMS message {} marked as read", message_id);
            }
        }
        Ok(())
    }
}
