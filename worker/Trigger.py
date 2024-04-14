
class Trigger:
    def __init__(self, name:str):
        self.name: str = name

from croniter import croniter
class CronTrigger(Trigger):
    def __init__(self, name:str, cron:str):
        super(CronTrigger, self).__init__(name)
        self.cron = croniter(cron) 

    def get_next(self):
        return self.cron.get_next()

class JobTrigger(Trigger):
    def __init__(self, name:str, job_name:str):
        super(JobTrigger, self).__init__(name)
        self.job_name = job_name
       
    def get_status(self):
       # Get the status of the given job and if it should trigger 

