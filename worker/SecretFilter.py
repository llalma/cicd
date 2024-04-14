import re
import sys

class SecretFilter(object):
    def __init__(self, filter_text: [str], stream):
        self.stream = stream
        self.filter_strings = filter_text
    
    def write(self, data):
        for s in self.filter_strings:
            data = re.sub(s, "*******", data)
        self.stream.write(data)
        self.stream.flush()

    def flush(self):
        self.stream.flush()

class MaskSecrets:
    def __init__(self, secrets: [str]):
        if not isinstance(secrets, list):
            secrets = [secrets]
        self.secrets = secrets
        
    def __enter__(self):
        # Create masks for stdout and stderr and redirect
        sys.stdout = SecretFilter(self.secrets, sys.stdout) 
        sys.stderr = SecretFilter(self.secrets, sys.stderr) 

    def __exit__(self, exc_type, exc_val, exc_tb):
        # Revert back to normal logging
        sys.stdout = sys.__stdout__
        sys.stderr = sys.__stderr__

