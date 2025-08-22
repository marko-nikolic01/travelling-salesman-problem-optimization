from fastapi import FastAPI
from app.controller import result_router

def create_app():
    app = FastAPI(title="Travelling salesman problem result presenter")

    app.include_router(result_router)

    return app

app = create_app()
