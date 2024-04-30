locals {
  output_path = "/tmp/CICD_test/layer/python"
}

###
# Create the lambda layer
###
resource "null_resource" "pip_install" {
  triggers = {
    shell_hash = "${sha256(file("${path.module}/lambda_function/requirements.txt"))}"
  }

  provisioner "local-exec" {
    working_dir = "lambda_function"
    command     = "python3 -m pip install -r requirements.txt -t ${local.output_path}/python --upgrade"
  }
}

###
# Create the zip file
###
data "archive_file" "layer" {
  type        = "zip"
  source_dir  = local.output_path
  output_path = "/tmp/CICD_test/lambda_layer.zip"
  depends_on  = [null_resource.pip_install]
}

###
# Create the lambda layer
###
resource "aws_lambda_layer_version" "layer" {
  layer_name          = "CICD_Layer"
  filename            = data.archive_file.layer.output_path
  source_code_hash    = data.archive_file.layer.output_base64sha256
  compatible_runtimes = ["python3.11"]
}
