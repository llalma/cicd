from SecretFilter import MaskSecrets

import pytest
from io import StringIO
import sys

@pytest.mark.parametrize("input_str, secrets, expected", 
    [("here secret_value,", ["secret_value"], "here *******,"),
     ("not a value to hide", ["no matches"], "not a value to hide")
    ])
def test_mask_stdout(input_str: str, secrets: [str], expected:str):
    try:
        sys.stdout = mystdout = StringIO()

        with MaskSecrets(secrets):
            print(input_str)
        
        logs = mystdout.getvalue()
        assert logs.strip() == expected.strip()

    except:
        assert False
    finally:
        sys.stdout = sys.__stdout__

@pytest.mark.parametrize("input_str, secrets, expected", 
    [("here secret_value,", ["secret_value"], "here *******,"),
     ("not a value to hide", ["no matches"], "not a value to hide")
    ])
def test_mask_stderr(input_str: str, secrets: [str], expected:str):
    try:
        sys.stderr = mystderr = StringIO()

        with MaskSecrets(secrets):
            print(input_str, file=sys.stderr)

        logs = mystderr.getvalue()
        assert logs.strip() == expected.strip()

    except:
        assert False
    finally:
        sys.stderr = sys.__stderr__
