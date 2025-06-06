import requests

url = "http://127.0.0.1:3000/greet"

data = {
    "name": "Alice",
    "age": 30
}

response = requests.post(url, json=data)

print("Status code:", response.status_code)
print("Response JSON:", response.json())
