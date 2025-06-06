import os 
import json 
import jwt 
import imaplib
import requests
import time 
#>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        #FastAPI 
import uvicorn
from typing import Union 
from fastapi import FastAPI,File,UploadFile,Request,Form
from fastapi.responses import HTMLResponse,RedirectResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
#>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

app = FastAPI() 

@app.get("/")
def get_requesttest():

     return {} 

@app.post("/post_fastapi_server")
async def post_fastapiserver(request:Request):
      reqdat = await request.json()
      print("Information post JSON",reqdat)  
      return reqdat 
if __name__ == "__main__":

       uvicorn.run("fastapi_postserver_test_rust:app",host="0.0.0.0",port=9000)
     





