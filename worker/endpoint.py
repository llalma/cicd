from fastapi import FastAPI
from pydantic import BaseModel


class Job(BaseModel):
    id: str
    job_name: str
    trigger: str
    commit: str


app = FastAPI()





@app.post("/")
async def create_item(job: Job):
    print(f"test")
    print(f"{job.id}")
    return job
