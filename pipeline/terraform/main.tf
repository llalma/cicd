###
# IAM roles
###
data "aws_iam_policy_document" "assume_role" {
  statement {
    effect = "Allow"

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }

    actions = ["sts:AssumeRole"]
  }
}

resource "aws_iam_role" "iam_for_lambda" {
  name               = "iam_for_lambda"
  assume_role_policy = data.aws_iam_policy_document.assume_role.json
}

###
# Create zip folder
###
data "archive_file" "lambda" {
  type        = "zip"
  source_dir  = "lambda_function"
  output_path = "/tmp/CICD_test/lambda_function.zip"
}

###
# Create the lambda function
###
resource "aws_lambda_function" "test_lambda" {

  function_name = "CICD_Pipeline"
  runtime       = "python3.11"

  # Stuff related to the actual python code
  filename         = data.archive_file.lambda.output_path
  source_code_hash = data.archive_file.lambda.output_base64sha256
  handler          = "main.lambda_handler"

  role = aws_iam_role.iam_for_lambda.arn

  layers = [aws_lambda_layer_version.layer.arn]

  environment {
    variables = {
      CICD_REDIS_IP   = "1.1.1.1"
      CICD_REDIS_PORT = "1234"
    }
  }
}
