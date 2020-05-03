#![windows_subsystem = "windows"]
use web_view::*;
use serde::{Serialize, Deserialize};
use clap::{App, Arg, SubCommand};

mod switcher;
mod utils;

#[derive(Serialize, Deserialize, Debug)]
struct HtmlResponse<'a> {
    r#type: &'a str,
    args: Vec<String>
}

#[tokio::main]
async fn main() {
    let matches = App::new("Akatsuki Switcher cli")
        .subcommand(
            SubCommand::with_name("switch")
                .about("Switch between servers")
                .help(
                    "Switch to either `bancho` or `akatsuki` or custom defined by --ip.",
                )
                .subcommand(
                    SubCommand::with_name("akatsuki")
                )
                .subcommand(
                    SubCommand::with_name("bancho")
                )
                .arg(
                    Arg::with_name("ip")
                        .short("i")
                        .help("Switch to custom servers for debugging purposes only."),
                ),
        )
        .subcommand(
            SubCommand::with_name("certificate")
                .about("Install Certificate")
                .subcommand(
                    SubCommand::with_name("install")
                )
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .help("Verbose logging output")
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("switch") {
        if !utils::is_admin() {
            println!("This program can't be executed as a non Administrator");
            std::process::exit(0);
        }

        if matches.is_present("akatsuki") {
            switcher::switch_to(switcher::fetch_ip().await.unwrap().trim()).unwrap();
        } else if matches.is_present("bancho") {
            switcher::remove_old().unwrap();
        } else if matches.is_present("both") {
            switcher::switch_to(switcher::fetch_ip().await.unwrap().trim()).unwrap();
            switcher::install_cert(switcher::fetch_cert().await.unwrap().trim()).await;
        } else {
            println!("try using IP");
        }

        std::process::exit(0);
    }

    if let Some(matches) = matches.subcommand_matches("certificate") {
        if !utils::is_admin() {
            println!("This program can't be executed as a non Administrator");
            std::process::exit(0);
        }

        if matches.is_present("install") {
            switcher::install_cert(switcher::fetch_cert().await.unwrap().trim()).await;
        }  else {
            println!("install is required!");
        }

        std::process::exit(0);
    }

    println!("{}", utils::is_admin());
    if utils::is_admin()
    {
        // Print Toast in case somebody did manage to run it as admin and it broke.
        #[cfg(windows)]
        {
            use winrt::*;
            use winrt::windows::data::xml::dom::*;
            use winrt::windows::ui::notifications::*;

            let toast_xml = ToastNotificationManager::get_template_content(ToastTemplateType::ToastText02).unwrap().unwrap();

            let toast_text_elements = toast_xml.get_elements_by_tag_name(&FastHString::new("text")).unwrap().unwrap();

            toast_text_elements.item(0).unwrap().unwrap().append_child(&toast_xml.create_text_node(&FastHString::new("Pws nyo :c")).unwrap().unwrap().query_interface::<IXmlNode>().unwrap()).unwrap();
            toast_text_elements.item(1).unwrap().unwrap().append_child(&toast_xml.create_text_node(&FastHString::new("Pwease do nyot wun this appwication as Adminyistwatow >w<  othewwise ouw fancy fwontend bweaks :c")).unwrap().unwrap().query_interface::<IXmlNode>().unwrap()).unwrap();

            let toast = ToastNotification::create_toast_notification(&toast_xml).unwrap();

            ToastNotificationManager::create_toast_notifier_with_id(&FastHString::new("Akatsuki Switcher")).unwrap().unwrap().show(&toast).unwrap();

            std::thread::sleep(std::time::Duration::from_secs(5));
            return;
        }
    }

    web_view::builder()
        .title("Akatsuki Switcher")
        .content(Content::Html(include_str!("../views/index.html")))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            let response: HtmlResponse = serde_json::from_str(&arg).unwrap();
            match response.r#type {
                "perform_switch" => {
                    let is_switched = response.args[2] == "true";

                    if is_switched {
                        webview.eval("setState('uninstall_ip')").unwrap();
                    } else {
                        webview.eval("setState('install_ip')").unwrap();
                    }
                    
                    
                    if !utils::host_run_as_admin(!is_switched) {
                        webview.eval("showNotification('error_install_ip')").unwrap();
                        
                        return Ok(())
                    }
                    
                    if !is_switched {
                        webview.eval("setState('install_cert')").unwrap();

                        if !utils::cert_run_as_admin() {
                            webview.eval("showNotification('error_install_cert')").unwrap();
    
                            return Ok(())
                        }
                    }
                
                    webview.eval(format!("callbackCheckStatus({})", switcher::is_switched().unwrap()).as_str()).unwrap();
                    
                    webview.eval("showNotification('ok')").unwrap();

                    Ok(())
                }
                "open_browser" => {
                    webbrowser::open(response.args[0].as_str()).unwrap();

                    Ok(())
                }
                "check_status" => {
                    webview.eval(format!("callbackCheckStatus({})", switcher::is_switched().unwrap()).as_str()).unwrap();
                    Ok(())
                }
                _ => unimplemented!("{}", arg)
            }
        })
        .run()
        .unwrap();
}
