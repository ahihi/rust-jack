extern crate libc;
extern crate jack;
use jack::{JackClient,JackPort};

#[test]
fn new_client_and_close() {
    let client = JackClient::open("new_client",jack::JackNullOption);
    assert!(client.close());
}

#[test]
fn client_name_size() {
    let size = JackClient::name_size();
    assert_eq!(size, 64);
}

#[test]
fn get_client_name() {
    let name = "check_me";
    let client = JackClient::open(name,jack::JackNullOption);
    let get_name = client.get_name();
    client.close();
    assert_eq!(get_name, name);
}

#[test]
fn activate() {
    let client = JackClient::open("activate_client",jack::JackNullOption);
    assert!(client.activate());
    assert!(client.deactivate());
    assert!(client.close());
}

#[test]
fn buffer_size_test() {
    let client = JackClient::open("buffer_size_test", jack::JackNullOption);
    let orig_size = client.get_buffer_size();
    assert!(0 < orig_size);
    let new_size = orig_size / 2;
    assert!(client.set_buffer_size(new_size));
    let actual_new_size = client.get_buffer_size();
    assert_eq!(actual_new_size, new_size);
    assert!(client.set_buffer_size(orig_size));
}

#[test]
fn cpu_load_test() {
    let client = JackClient::open("cpu_load_test", jack::JackNullOption);
    let load = client.cpu_load();
    assert!(0.0 < load && load <= 100.0);
}

#[test]
fn port_test() {
    let client = JackClient::open("port_test",jack::JackNullOption);
    let port = client.register_port("test_port",
                                    jack::JACK_DEFAULT_AUDIO_TYPE,
                                    jack::JackPortIsOutput | jack::JackPortIsTerminal,
                                    0);
    assert_eq!(port.name(), "port_test:test_port");
    assert_eq!(port.short_name(), "test_port");
    assert!(port.flags() == jack::JackPortIsTerminal | jack::JackPortIsOutput);
    assert_eq!(port.get_type(), jack::JACK_DEFAULT_AUDIO_TYPE);
    assert!(client.port_is_mine(port));
    assert_eq!(port.connected(), 0);
    client.unregister_port(&port);
    assert!(client.close());
}

#[test]
fn port_connect_test() {
    let client = JackClient::open("port_connect_test",jack::JackNoStartServer);
    let in_port = client.register_port("input_test",
                                       jack::JACK_DEFAULT_AUDIO_TYPE,
                                       jack::JackPortIsInput,
                                       0);
    let out_port = client.register_port("output_test",
                                        jack::JACK_DEFAULT_AUDIO_TYPE,
                                        jack::JackPortIsOutput,
                                        0);

    client.activate(); // need to be activated to connect ports

    let res = client.connect("port_connect_test:output_test",
                             "port_connect_test:input_test");
    match res {
        Ok(_) => {}
        Err(s) => panic!(s)
    }

    assert_eq!(in_port.connected(), 1);
    assert_eq!(out_port.connected(), 1);

    let conns = in_port.get_connections();
    assert_eq!(conns[0], "port_connect_test:output_test");

    assert!(client.disconnect("port_connect_test:output_test",
                              "port_connect_test:input_test"));

    assert_eq!(in_port.connected(), 0);
    assert_eq!(out_port.connected(), 0);

    let noconns = in_port.get_connections();
    assert_eq!(noconns.len(), 0);

    assert!(client.close());
}



#[test]
fn port_type_size() {
    assert_eq!(JackPort::type_size(), 32); // Might fail if this changes
}

#[test]
fn port_alias() {
    let client = JackClient::open("port_alias_test",jack::JackNoStartServer);
    let in_port = client.register_port("alias_test",
                                       jack::JACK_DEFAULT_AUDIO_TYPE,
                                       jack::JackPortIsInput,
                                       0);
    assert!(in_port.set_alias("alias1"));
    let aliases = in_port.get_aliases();
    assert_eq!(aliases.len(), 1);
    assert_eq!(aliases[0], "alias1");
    assert!(in_port.set_alias("alias2"));
    let aliases2 = in_port.get_aliases();
    assert_eq!(aliases2.len(), 2);
    assert_eq!(aliases2[0], "alias1");
    assert_eq!(aliases2[1], "alias2");
    assert!(client.close());
}
