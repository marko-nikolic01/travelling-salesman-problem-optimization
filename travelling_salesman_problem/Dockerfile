FROM python:3.11-slim

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY travelling_salesman_problem ./travelling_salesman_problem
RUN chmod +x ./travelling_salesman_problem/*

COPY ./app ./app

ENV PYTHONPATH=/app
CMD ["python", "app/main.py"]
