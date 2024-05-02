import CICD_TBD
import json


def lambda_handler(event, context):

    # with CICD_TBD.Pipeline("pipe") as pipeline:
    #     with CICD_TBD.Stage("stage 1") as stage:
    #         print(stage.name)
    #     with CICD_TBD.Stage("stage 2") as stage:
    #         print(stage.name)

    return {"statusCode": 200, "body": json.dumps("Hello from Lambda!")}


if __name__ == "__main__":
    lambda_handler(0, 0)
