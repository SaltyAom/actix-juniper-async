wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"
wrk.body = "{\"query\":\"{\\n  getAllProduct {\\n    name,\\n    kind\\n  }\\n}\",\"variables\":{},\"operationName\":\"\"}"