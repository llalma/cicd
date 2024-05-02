
data "github_repository" "repo" {
  full_name = local.repo
}

resource "github_repository_webhook" "foo" {
  repository = data.github_repository.repo.name

  configuration {
    url          = aws_lambda_function_url.endpoint.function_url
    content_type = "json"
    insecure_ssl = false
  }

  active = true 

  events = ["push"]
}

