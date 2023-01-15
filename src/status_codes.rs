pub fn not_found_404() -> String {
//fn not_found_404(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let contents = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Oops!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>
"#;

    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    //stream.write_all(response.as_bytes()).unwrap();
    return response
}


//fn unauthorized_401(mut stream: TcpStream) {
pub fn unauthorized_401() -> String {
    let status_line = "HTTP/1.1 401 Unauthorized";
    let contents = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Unauthorized!</title>
  </head>
  <body>
    <h1>Unauthorized!</h1>
    <p>Sorry...</p>
  </body>
</html>
"#;

    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    //stream.write_all(response.as_bytes()).unwrap();
    return response
}
