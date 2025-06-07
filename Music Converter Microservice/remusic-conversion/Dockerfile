# Necessary for GPU acceleration
FROM nvidia/cuda:11.2.2-cudnn8-runtime-ubuntu20.04

# Avoid interactive elements on setup
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Etc/UTC

WORKDIR /app

RUN apt-get update && \
    apt-get install -y python3.10 python3-pip python3-venv wget && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN python3 -m venv /opt/venv && \
    /opt/venv/bin/pip install --upgrade pip

ENV PATH="/opt/venv/bin:$PATH"

COPY requirements.txt .

RUN pip install --no-cache-dir -r requirements.txt

COPY src/ .

RUN mkdir -p /usr/local/lib/python3.10/site-packages/oemer/checkpoints/unet_big && \
    mkdir -p /usr/local/lib/python3.10/site-packages/oemer/checkpoints/seg_net && \
    # Download checkpoints
    wget -O 1st_model.onnx https://github.com/BreezeWhite/oemer/releases/download/checkpoints/1st_model.onnx && \
    wget -O 1st_weights.h5 https://github.com/BreezeWhite/oemer/releases/download/checkpoints/1st_weights.h5 && \
    wget -O 2nd_model.onnx https://github.com/BreezeWhite/oemer/releases/download/checkpoints/2nd_model.onnx && \
    wget -O 2nd_weights.h5 https://github.com/BreezeWhite/oemer/releases/download/checkpoints/2nd_weights.h5 && \
    # Move and rename as required
    mv 1st_model.onnx /usr/local/lib/python3.10/site-packages/oemer/checkpoints/unet_big/model.onnx && \
    mv 1st_weights.h5 /usr/local/lib/python3.10/site-packages/oemer/checkpoints/unet_big/weights.h5 && \
    mv 2nd_model.onnx /usr/local/lib/python3.10/site-packages/oemer/checkpoints/seg_net/model.onnx && \
    mv 2nd_weights.h5 /usr/local/lib/python3.10/site-packages/oemer/checkpoints/seg_net/weights.h5

CMD ["python3", "-u", "conversion_script.py"]