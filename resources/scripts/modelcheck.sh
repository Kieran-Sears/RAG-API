#!/bin/sh

# Get the Ollama API endpoint from the environment variable
OLLAMA_API_ENDPOINT=${OLLAMA_API_ENDPOINT:-"http://ollama:11434"}

# Function to check for the model in the response
check_model() {
  curl -s "${OLLAMA_API_ENDPOINT}/api/tags" | jq -e '.models[] | select(.name == "llama3.1")' > /dev/null
}

# Function to pull the model and log progress
pull_model_and_log_progress() {
  last_progress=0

  curl -X POST "${OLLAMA_API_ENDPOINT}/api/pull" -d '{
    "name": "llama3.1",
    "stream": true
  }' -H "Content-Type: application/json" | while IFS= read -r line; do
    # Check if the line contains "total" and "completed"
    if echo "$line" | jq -e 'has("total") and has("completed")' > /dev/null; then
      status=$(echo "$line" | jq -r '.status')
      total=$(echo "$line" | jq -r '.total // 0')
      completed=$(echo "$line" | jq -r '.completed // 0')
      if [ "$total" -gt 0 ]; then
        progress=$(( completed * 100 / total ))
        # Print progress if it has increased by at least 2% since the last update
        if [ "$progress" -ge $(( last_progress + 2 )) ] || [ "$progress" -eq 100 ]; then
          echo "Pulling progress: $progress% ($completed/$total)"
          last_progress=$progress
        fi
      fi
    else
      # Print statuses that do not include "total" or "completed"
      echo "$line"
    fi

    # Check if the status indicates completion
    status=$(echo "$line" | jq -r '.status')
    if [ "$status" = "success" ]; then
      echo "Model llama3.1 has been pulled successfully."
      return 0
    fi
  done

  echo "Failed to pull model llama3.1."
  return 1
}

# Check if the model is available
if check_model; then
  echo "Model llama3.1 is already available."
else
  echo "Model llama3.1 not found. Initiating pull request."
  if pull_model_and_log_progress; then
    echo "Model pull completed successfully."
  else
    echo "Model pull failed."
    exit 1
  fi
fi
