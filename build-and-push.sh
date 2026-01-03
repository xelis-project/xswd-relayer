#!/bin/bash
# Build and push xswd-relayer Docker image

set -e

# Configuration
IMAGE_NAME="xswd-relayer"
VERSION="${1:-latest}"
REGISTRY="${DOCKER_REGISTRY:-}"  # Set DOCKER_REGISTRY env var or pass as argument

# Color output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Building xswd-relayer Docker image...${NC}"

# Build the image
docker build -t ${IMAGE_NAME}:${VERSION} .

if [ -n "$REGISTRY" ]; then
    # Tag for registry
    FULL_IMAGE="${REGISTRY}/${IMAGE_NAME}:${VERSION}"
    docker tag ${IMAGE_NAME}:${VERSION} ${FULL_IMAGE}

    # Also tag as latest if version is specified
    if [ "$VERSION" != "latest" ]; then
        docker tag ${IMAGE_NAME}:${VERSION} ${REGISTRY}/${IMAGE_NAME}:latest
    fi

    echo -e "${BLUE}Pushing to registry: ${FULL_IMAGE}${NC}"
    docker push ${FULL_IMAGE}

    if [ "$VERSION" != "latest" ]; then
        docker push ${REGISTRY}/${IMAGE_NAME}:latest
    fi

    echo -e "${GREEN}✓ Successfully pushed ${FULL_IMAGE}${NC}"
else
    echo -e "${GREEN}✓ Successfully built ${IMAGE_NAME}:${VERSION}${NC}"
    echo -e "${BLUE}To push to registry, set DOCKER_REGISTRY environment variable${NC}"
    echo -e "${BLUE}Example: DOCKER_REGISTRY=yourusername ./build-and-push.sh v0.1.0${NC}"
fi

echo -e "${GREEN}Done!${NC}"
