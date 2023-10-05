import redis

class RedisController:
    def __init__(self):
        self.r = redis.Redis(host='localhost', port=6379, decode_responses=True)
    
    def find_email(self, email: str = ""):
        self.r.get()

    def delete_email(self):
        pass
