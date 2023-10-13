
previewVersion=11
quickstartHash=testing@sha256:263ab2e416e72b068ea84e4db09971a733b885121225518004520f27f8effa61
 

echo "Searching for a previous soroban-preview docker container"
containerID=$(docker ps --filter=`name=soroban-preview-${previewVersion}` --all --quiet)
if [[ ${containerID} ]]; then
    echo "Start removing soroban-preview-${previewVersion}  container."
    docker rm --force soroban-preview-${previewVersion}
    echo "Finished removing soroban-preview-${previewVersion} container."
else
    echo "No previous soroban-preview-${previewVersion} container was found"
fi

currentDir=$(pwd)
docker run -dti \
  --volume ${currentDir}:/workspace \
  --name soroban-preview-${previewVersion} \
  -p 8001:8000 \
  --ipc=host \
  esteblock/soroban-preview:${previewVersion}


previewVersion=11
docker exec -it soroban-preview-${previewVersion} bash
