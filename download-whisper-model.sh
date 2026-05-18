#!/bin/bash
# download-whisper-model.sh

MODEL_DIR="$HOME/.lingomate/models"
MODEL_FILE="ggml-small.bin"
#MODEL_URL="https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin"
MODEL_URL="https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin"

echo "Creating model directory: $MODEL_DIR"
mkdir -p "$MODEL_DIR"

if [ -f "$MODEL_DIR/$MODEL_FILE" ]; then
    echo "Model already exists: $MODEL_DIR/$MODEL_FILE"
    echo "File size: $(du -h "$MODEL_DIR/$MODEL_FILE" | cut -f1)"
    exit 0
fi

echo "Downloading Whisper model: $MODEL_FILE"
echo "URL: $MODEL_URL"
echo "This may take a few minutes (model size: ~150MB)..."

curl -L "$MODEL_URL" -o "$MODEL_DIR/$MODEL_FILE"

if [ $? -eq 0 ]; then
    echo "✓ Model downloaded successfully!"
    echo "Location: $MODEL_DIR/$MODEL_FILE"
    echo "Size: $(du -h "$MODEL_DIR/$MODEL_FILE" | cut -f1)"
else
    echo "✗ Failed to download model"
    exit 1
fi