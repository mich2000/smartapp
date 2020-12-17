docker build -t xam-id --build-arg JWT_FILE_PATH=$1 --build-arg ENV_FILE_PATH=$2 -f ./backend/Auth.dockerfile .
docker build -t xam-bis JWT_FILE_PATH=$1 --build-arg ENV_FILE_PATH=$3 -f ./backend/Bis.dockerfile .
docker build -t xam-web -f ./frontend/xam-xam-react/React.dockerfile .