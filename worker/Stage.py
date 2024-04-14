from typing import Callable

class Stage:
    def __init__(self, index: int, function: Callable, opts: dict()):
        self.func = function 
        self.opts = opts
        self.index = index
    
    def get_name(self):
        return self.opts['name']
    
    def run(self, *args):
        return self.func(*args)
