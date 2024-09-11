FROM rust:slim-buster AS build

# Устанавливаем необходимые зависимости для сборки
RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install libpq-dev

# Устанавливаем рабочую директорию
WORKDIR /app

# Копируем все файлы проекта в контейнер
COPY . .

# Сборка приложения в режиме релиза
RUN cargo build --release

# Финальный образ для запуска
FROM ubuntu:latest AS runtime

# Устанавливаем зависимости для выполнения
RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install libpq-dev

# Устанавливаем рабочую директорию
WORKDIR /app

# Копируем собранный бинарный файл из этапа сборки
COPY --from=build /app/target/release/order_data_demo_service /app/order_data_demo_service

# Меняем права на выполнение бинарника
RUN chmod +x /app/order_data_demo_service

# Используем непривилегированного пользователя
USER nobody

# Открываем порт для приложения
EXPOSE 8000

# Запуск приложения
ENTRYPOINT ["/app/order_data_demo_service"]