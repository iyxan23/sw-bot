import psycopg2
import os

DATABASE_CREDENTIALS = os.environ['DATABASE_CREDENTIALS']
connection = psycopg2.connect(DATABASE_CREDENTIALS)
