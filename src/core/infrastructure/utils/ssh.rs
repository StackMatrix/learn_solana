use std::io::Read;
use std::net::{IpAddr, TcpStream};
use ssh2::Session;

/// @Description ssh 管理
///
/// @Param nodes 节点列表
#[allow(dead_code)]
pub struct SSHManager {
    pub nodes: Vec<Node>
}

/// @Description 节点信息
///
/// @Param ip_addr 主机地址
///
/// @Param hostname 主机名
///
/// @Param username 用户名
///
/// @Param password 密码
///
/// @Param group 分组
#[allow(dead_code)]
pub struct Node {
    pub ip_addr: IpAddr,
    pub port: i64,
    pub hostname: String,
    pub username: String,
    pub password: String,
    pub group: String,
}

impl SSHManager {
    pub fn new() -> Self {
        SSHManager { nodes: Vec::new() }
    }

    /// @Description 添加节点
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    /// @Description 所有节点执行命令
    ///
    /// @Param command 要执行的命令
    pub fn execute_command_on_all_nodes(&self, command: &str) {
        for node in &self.nodes {
            println!("+[SSH] Executing command on node: {}", node.hostname);

            match self.execute_command(node, command) {
                Ok(output) => println!("+[SSH] Output from {}: {}", node.hostname, output),
                Err(e) => eprintln!("-[SSH] Error executing command on {}: {}", node.hostname, e),
            }
        }
    }

    /// @Description 执行命令
    ///
    /// @Param node 要执行的节点
    ///
    /// @Param command 要执行的命令
    fn execute_command(&self, node: &Node, command: &str) -> Result<String, String> {
        let tcp = TcpStream::connect((node.ip_addr, 22)).map_err(|e| e.to_string())?;
        let mut sess = Session::new().map_err(|e| e.to_string())?;
        sess.set_tcp_stream(tcp);
        sess.handshake().map_err(|e| e.to_string())?;
        sess.userauth_password(&node.username, &node.password).map_err(|e| e.to_string())?;

        if !sess.authenticated() {
            return Err("-[SSH] Failed to authenticate with SSH server".to_string());
        }

        let mut channel = sess.channel_session().map_err(|e| e.to_string())?;
        channel.exec(command).map_err(|e| e.to_string())?;

        let mut s = String::new();
        channel.read_to_string(&mut s).map_err(|e| e.to_string())?;

        channel.send_eof().map_err(|e| e.to_string())?;
        channel.wait_close().map_err(|e| e.to_string())?;

        Ok(s)
    }
}


#[test]
fn test() {
    let mut manager = SSHManager::new();

    manager.add_node(Node {
        ip_addr: "127.0.0.1".parse().unwrap(),
        port: 23,
        hostname: "node1".to_string(),
        username: "stack".to_string(),
        password: "123qweasdzxc".to_string(),
        group: "group1".to_string(),
    });

    manager.execute_command_on_all_nodes("ls -la");
}