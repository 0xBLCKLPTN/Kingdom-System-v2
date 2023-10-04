import smtplib, ssl

message = """\
Subject: Hi there

This message is sent from Python."""

class Client:
    def __init__(self, smtp_server: str, sender: str, password: str, port: str = 587):
        self.port = port
        self.smtp_server = smtp_server
        self.context = ssl.create_default_context()
        self.sender = sender
        self.password = password 

    def send_message(self, receiver: str):
        with smtplib.SMTP(self.smtp_server, self.port) as server:
            server.ehlo()  # Can be omitted
            server.starttls(context=self.context)
            server.ehlo()  # Can be omitted
            server.login(self.sender, self.password)
            server.sendmail(self.sender, receiver, message)


if __name__ == "__main__":
    client = Client(port=587, smtp_server="smtp.gmail.com", sender="EMAIL", password="PASSWORD")
    client.send_message(receiver="blcklptn@icloud.com")
