use std::io::BufRead;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;
use std::thread;
use std::process::Command;

const SERVER_ADDRESS: &str = "localhost";
const PORT: u32 = 8080;

const DECODER_IP_ADDRESS: &str = "192.168.1.100";
const DECODER_MAC_ADDRESS: &str = "xx:xx:xx:xx:xx:xx";

const BOX_SN: &str = "XXXXXXXXXXXXXXXXXX";
const BOX_MAC_ADDRESS: &str = "xx:xx:xx:xx:xx:xx";
const BOX_IDUR: &str = "XXXXXXX";

fn main() {
    let listener = TcpListener::bind(format!("{}:{:?}", SERVER_ADDRESS, PORT)).unwrap();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(move || {
                handle_client(stream);
            });
        }
    }
}

fn handle_client<T: Read + Write>(mut stream: T) {
    let buffer = read_header(&mut stream);
    let request_string = str::from_utf8(&buffer).unwrap();

    if request_string.is_empty() {
        return;
    }

    let mut parts = request_string.split(' ');

    let _method = parts.next().unwrap().trim();
    let path = parts.next().unwrap().trim();

    let response: String;
    // Don't forget to add content length, and other api stuff

    match path {
        "/api/1.0/?method=system.getInfo" => {
            response = system_get_info()
        },
        "/api/1.0/?method=ftth.getInfo" => {
            response = ftth_get_info()
        },
        "/api/1.0/?method=lan.getHostsList" => {
            response = lan_get_hosts_list()
        },
        "/api/1.0/?method=wan.getInfo" => {
            response = wan_get_info()
        },
        _ => {
            response = default_response()
        },
    }

    let bytes = response.as_bytes().to_vec();
    stream.write_all(&bytes).unwrap();
    stream.flush().unwrap();
}

fn read_header<T: Read + Write>(stream: &mut T) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut reader = std::io::BufReader::new(stream);
    loop {
        reader.read_until(b'\n', &mut buffer).unwrap();
        if buffer.ends_with(b"\r\n\r\n") {
            break;
        }
    }
    buffer
}

fn system_get_info() -> String {
    let uptime = Command::new("cat").arg("/proc/uptime").output().expect("Error with cat /proc/uptime command");
    let temp_uptime = match str::from_utf8(&*uptime.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let current_uptime = temp_uptime.split(' ').next().unwrap().trim().split('.').next().unwrap().trim();

    let date = Command::new("date").arg("+%Y%m%d%H%M").output().expect("Error with date command");
    let current_datetime = match str::from_utf8(&*date.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let date = Command::new("date").arg("+%a, %d %b %Y %X %Z").output().expect("Error with date command");
    let current_date = match str::from_utf8(&*date.stdout) {
        Ok(v) => v.trim(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let payload = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
    <rsp stat=\"ok\" version=\"1.0\">\
        <system product_id=\"NB6VAC-FXC-r0\" \
            serial_number=\"{}\" \
            mac_addr=\"{}\" \
            net_mode=\"router\" \
            net_infra=\"ftth\" \
            uptime=\"{}\" \
            version_mainfirmware=\"NB6VAC-MAIN-R4.0.44j\" \
            version_rescuefirmware=\"NB6VAC-MAIN-R4.0.44i\" \
            version_bootloader=\"NB6VAC-BOOTLOADER-R4.0.8\" \
            version_dsldriver=\"NB6VAC-XDSL-A2pv6F039p\" \
            current_datetime=\"{}\" \
            refclient=\"\" \
            idur=\"{}\" \
            alimvoltage=\"12251\" \
            temperature=\"48399\" \
         />\
    </rsp>", BOX_SN, BOX_MAC_ADDRESS, current_uptime, current_datetime, BOX_IDUR);

    return format!("HTTP/1.1 200 OK\r\n\
    Content-Length: {}\r\n\
    Content-Type: text/xml; charset=utf-8\r\n\
    Date: {}\r\n\
    Server: Rust\r\n\
    \r\n{}", payload.len(), current_date, payload);
}

fn ftth_get_info() -> String {
    let date = Command::new("date").arg("+%a, %d %b %Y %X %Z").output().expect("Error with date command");
    let current_date = match str::from_utf8(&*date.stdout) {
        Ok(v) => v.trim(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let payload = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
    <rsp stat=\"ok\" version=\"1.0\">\
        <ftth status=\"up\" wanfibre=\"in\"/>\
    </rsp>";

    return format!("HTTP/1.1 200 OK\r\n\
    Content-Length: {}\r\n\
    Content-Type: text/xml; charset=utf-8\r\n\
    Date: {}\r\n\
    Server: Rust\r\n\
    \r\n{}", payload.len(), current_date, payload);
}

fn lan_get_hosts_list() -> String {
    let date = Command::new("date").arg("+%a, %d %b %Y %X %Z").output().expect("Error with date command");
    let current_date = match str::from_utf8(&*date.stdout) {
        Ok(v) => v.trim(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let payload = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
    <rsp stat=\"ok\" version=\"1.0\">\
        <host type=\"stb\" \
            name=\"STB7\" \
            ip=\"{}\" \
            mac=\"{}\" \
            iface=\"lan3\" \
            probe=\"56\" \
            alive=\"350261\" \
            status=\"online\" \
        />\
    </rsp>", DECODER_IP_ADDRESS, DECODER_MAC_ADDRESS);

    return format!("HTTP/1.1 200 OK\r\n\
    Content-Length: {}\r\n\
    Content-Type: text/xml; charset=utf-8\r\n\
    Date: {}\r\n\
    Server: Rust\r\n\
    \r\n{}", payload.len(), current_date, payload);
}

fn wan_get_info() -> String {
    let uptime = Command::new("cat").arg("/proc/uptime").output().expect("Error with cat /proc/uptime command");
    let temp_uptime = match str::from_utf8(&*uptime.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let current_uptime = temp_uptime.split(' ').next().unwrap().trim().split('.').next().unwrap().trim();

    let address = Command::new("curl").arg("http://ifconfig.me/ip").output().expect("Error with curl command");
    let current_address = match str::from_utf8(&*address.stdout) {
        Ok(v) => v,
        Err(..) => "",
    };

    let date = Command::new("date").arg("+%a, %d %b %Y %X %Z").output().expect("Error with date command");
    let current_date = match str::from_utf8(&*date.stdout) {
        Ok(v) => v.trim(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let payload = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
    <rsp stat=\"ok\" version=\"1.0\">\
        <wan status=\"up\" \
            uptime=\"{}\" \
            ip_addr=\"{}\" \
            infra=\"ftth\" \
            mode=\"ftth/routed\" \
            infra6=\"\" \
            status6=\"down\" \
            uptime6=\"\" \
            ipv6_addr=\"\" \
        />\
    </rsp>", current_uptime, current_address);

    return format!("HTTP/1.1 200 OK\r\n\
    Content-Length: {}\r\n\
    Content-Type: text/xml; charset=utf-8\r\n\
    Date: {}\r\n\
    Server: Rust\r\n\
    \r\n{}", payload.len(), current_date, payload);
}

fn default_response() -> String {
    let date = Command::new("date").arg("+%a, %d %b %Y %X %Z").output().expect("Error with date command");
    let current_date = match str::from_utf8(&*date.stdout) {
        Ok(v) => v.trim(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    return format!("HTTP/1.1 200 OK\r\n\
    Content-Length: {}\r\n\
    Content-Type: text/xml; charset=utf-8\r\n\
    Date: {}\r\n\
    Server: Rust\r\n\
    \r\n\"\"", 0, current_date);
}