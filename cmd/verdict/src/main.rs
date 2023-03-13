use clap::{App, Arg};

pub mod client_api {
    tonic::include_proto!("clientapi");
}

mod opa;
mod resource;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    let matches = App::new("verdict")
        .version("0.1")
        .author("Inclavare-Containers Team")
        .arg(
            Arg::with_name("set_opa_policy")
                .long("set-opa-policy")
                .value_name("POLICY_NAME")
                .value_name("POLICY_PATH")
                .help("Generate a policy file named <POLICY_NAME>, according to the contents in <POLICY_PATH>.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("export_opa_policy")
                .long("export-opa-policy")
                .value_name("POLICY_NAME")
                .help("Export the contents of the policy file named <POLICY_NAME>.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("path")
                .long("path")
                .short("p")
                .value_name("PATH")
                .help("Specify the path of the export file, must be used with '-e'.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("set_opa_reference")
                .long("set-opa-reference")
                .value_name("REFERENCE_NAME")
                .value_name("REFERENCE_PATH")
                .help("Generate a reference file named <REFERENCE_NAME>, according to the contents in <REFERENCE_PATH>.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("export_opa_reference")
                .long("export-opa-reference")
                .value_name("REFERENCE_NAME")
                .help("export OPA reference file named <REFERENCE_NAME>")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("client_api")
                .long("client-api")
                .short("c")
                .value_name("CLIENT_API_ADDRESS")
                .help("Specify the client API's connection address.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("test_opa_remote")
                .long("test-opa-remote")
                .value_name("POLICY_NAME")
                .value_name("REFERENCE_NAME")
                .value_name("INPUT_PATH")
                .help("test OPA's remote policy and remote reference")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("test_opa_local")
                .long("test-opa-local")
                .value_name("POLICY_PATH")
                .value_name("REFERENCE_PATH")
                .value_name("INPUT_PATH")
                .help("test OPA's local policy and local reference")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("test_opa_local_policy")
                .long("test-opa-local-policy")
                .value_name("POLICY_PATH")
                .value_name("REFERENCE_NAME")
                .value_name("INPUT_PATH")
                .help("test OPA's local policy and remote reference")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("test_opa_local_reference")
                .long("test-opa-local-reference")
                .value_name("POLICY_NAME")
                .value_name("REFERENCE_PATH")
                .value_name("INPUT_PATH")
                .help("test OPA's remote policy and local reference")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("get_resource")
                .long("get_resource")
                .value_name("RESOURCE_ID")
                .help("get resource from verdictd with id <RESOURCE_ID>")
        )
        .arg(
            Arg::with_name("set_resource")
                .long("set_resource")
                .value_name("RESOURCE_ID")
                .value_name("RESOURCE_PATH")
                .help("set resource with id <RESOURCE_ID> according to the contents in <RESOURCE_PATH>.")
                .takes_value(true),
        )
        .get_matches();

    let client_api = if matches.is_present("client_api") {
        matches.value_of("client_api").unwrap().to_string()
    } else {
        "[::1]:60000".to_string()
    };
    info!("Connect to Verdictd with addr: {}", client_api);

    // set_opa_policy
    if matches.is_present("set_opa_policy") {
        opa::set_policy_cmd(
            matches.values_of("set_opa_policy").unwrap().collect(),
            &client_api,
        )
        .await;
    }

    // export_opa_policy
    if matches.is_present("export_opa_policy") {
        let mut path: String = if matches.is_present("path") {
            matches.value_of("path").unwrap().to_string()
        } else {
            "./".to_string()
        };
        if !path.ends_with("/") {
            path = format!("{}/", path);
        }
        opa::export_policy_cmd(
            matches.value_of("export_opa_policy").unwrap(),
            path,
            &client_api,
        )
        .await;
    }

    // set data
    if matches.is_present("set_opa_reference") {
        opa::set_reference_cmd(
            matches.values_of("set_opa_reference").unwrap().collect(),
            &client_api,
        )
        .await;
    }

    // export Data
    if matches.is_present("export_opa_reference") {
        let mut path: String = if matches.is_present("path") {
            matches.value_of("path").unwrap().to_string()
        } else {
            "./".to_string()
        };
        if !path.ends_with("/") {
            path = format!("{}/", path);
        }
        opa::export_reference_cmd(
            matches.value_of("export_opa_reference").unwrap(),
            path,
            &client_api,
        )
        .await;
    }

    if matches.is_present("test_opa_remote") {
        opa::test_remote_cmd(
            matches.values_of("test_opa_remote").unwrap().collect(),
            &client_api,
        )
        .await;
    }

    if matches.is_present("test_opa_local") {
        opa::test_local_cmd(
            matches.values_of("test_opa_local").unwrap().collect(),
            &client_api,
        )
        .await;
    }

    if matches.is_present("test_opa_local_policy") {
        opa::test_localpolicy_cmd(
            matches
                .values_of("test_opa_local_policy")
                .unwrap()
                .collect(),
            &client_api,
        )
        .await;
    }

    if matches.is_present("test_opa_local_reference") {
        opa::test_localreference_cmd(
            matches
                .values_of("test_opa_local_reference")
                .unwrap()
                .collect(),
            &client_api,
        )
        .await;
    }

    if matches.is_present("get_resource") {
        let resource_id = matches.value_of("RESOURCE_ID").unwrap();
        resource::get_resource_cmd(resource_id, &client_api).await;
    }

    if matches.is_present("set_resource") {
        let resource_id = matches.value_of("RESOURCE_ID").unwrap();
        let resource_path = matches.value_of("RESOURCE_PATH").unwrap();
        resource::set_resource_cmd(resource_path, resource_id, &client_api).await;
    }
}
