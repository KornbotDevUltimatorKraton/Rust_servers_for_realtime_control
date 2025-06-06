import requests 
from itertools import count 

for i in count(0):
    reqdat = requests.post("http://127.0.0.1:3000/greet",json={"name":"Alice","age":30}).json()
    print(reqdat)
