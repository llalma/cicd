from enum import Enum, auto

class Outcome(Enum):
    WAITING = auto()
    RUNNING = auto()
    SUCCESS = auto()
    ERROR = auto()
    UNSTABLE = auto()
    
