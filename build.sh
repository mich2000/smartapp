docker build -t xam-build -f ./backend/Build.dockerfile ./backend
docker build -t xam-run -f ./backend/Run.dockerfile ./backend
docker build -t xam-id --build-arg JWT_FILE_PATH=$1 -f ./backend/Auth.dockerfile ./backend
docker build -t xam-bis --build-arg  JWT_FILE_PATH=$1 -f ./backend/Bis.dockerfile ./backend
docker build -t xam-web -f ./frontend/React.dockerfile ./frontend
docker tag xam-id xam-id:staging
docker tag xam-web xam-web:staging
docker tag xam-bis xam-bis:staging