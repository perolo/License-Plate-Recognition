#FROM ghcr.io/astral-sh/uv:latest
FROM ghcr.io/astral-sh/uv:0.6.5-python3.13-bookworm

#RUN apt-get update && apt-get install -y python3-opencv python3-numpy python3-matplotlib python3-pip
#RUN apt-get update && apt-get install -y python3 curl python3-opencv python3-numpy python3-matplotlib

#RUN curl -LsSf https://astral.sh/uv/install.sh | sh

#RUN pip3 install --no-cache-dir --upgrade pip3 

#RUN pip3 install --break-system-packages --no-cache-dir easyocr imutils

# Copy the project into the image
ADD .. /app

# Change the working directory to the `app` directory
WORKDIR /app/python-uv


# Copy the lockfile and `pyproject.toml` into the image
#ADD ./python-uv/uv.lock /app/uv.lock
#ADD ./python-uv/pyproject.toml /app/pyproject.toml

# Install dependencies
RUN uv sync --frozen --no-install-project

# Sync the project
RUN uv sync --frozen

RUN apt-get update && apt-get install -y python3-opencv python3-numpy python3-matplotlib

# Place executables in the environment at the front of the path
ENV PATH="/app/.venv/bin:$PATH"

# Reset the entrypoint, don't invoke `uv`
ENTRYPOINT []

#CMD ["bash"]
