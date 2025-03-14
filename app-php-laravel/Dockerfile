FROM php:8.3-fpm-alpine

# Add PHP repository and update package list
RUN apk add --no-cache ca-certificates && \
    apk add --no-cache --repository=https://dl-cdn.alpinelinux.org/alpine/edge/community/ php83

# Install system dependencies and PHP extensions
RUN apk add --no-cache \
    nginx \
    supervisor \
    # Base utils
    bash \
    nano \
    htop \
    dcron \
    bash-completion \
    # PHP and extensions
    php83-fpm \
    php83-common \
    php83-gd \
    php83-mysqli \
    php83-pdo_mysql \
    php83-mbstring \
    php83-curl \
    php83-xml \
    php83-bcmath \
    php83-zip \
    php83-exif \
    php83-pdo_pgsql \
    php83-pgsql \
    php83-opcache \
    php83-calendar \
    php83-ctype \
    php83-ffi \
    php83-fileinfo \
    php83-ftp \
    php83-gettext \
    php83-intl \
    php83-phar \
    php83-posix \
    php83-session \
    php83-shmop \
    php83-sockets \
    php83-sysvmsg \
    php83-sysvsem \
    php83-sysvshm \
    php83-tokenizer \
    php83-pecl-xdebug \
    php83-pecl-amqp \
    # Runtime libraries
    libzip \
    libpng \
    oniguruma \
    libxml2 \
    curl \
    freetype \
    libjpeg-turbo \
    libgd \
    postgresql-libs \
    icu \
    gettext \
    libffi \
    rabbitmq-c \
    openssl

# Install Composer
COPY --from=composer:latest /usr/bin/composer /usr/bin/composer

# Configure PHP
COPY php/conf.d/custom.ini /etc/php83/conf.d/
COPY php/php-fpm.d/www.conf /etc/php83/php-fpm.d/

# Setup supervisor
COPY supervisor/supervisord.conf /etc/supervisord.conf

# Setup cron
RUN touch /var/log/cron.log && chmod 0777 /var/log/cron.log
COPY cron/schedule-cron /etc/cron.d/
RUN chmod 0644 /etc/cron.d/schedule-cron
RUN crontab /etc/cron.d/schedule-cron

# Copy nginx configuration
COPY nginx/app.conf /etc/nginx/http.d/default.conf

# Create necessary directories and set permissions
RUN mkdir -p /run/nginx /var/log/supervisor /app \
    && chown -R www-data:www-data /app \
    && chmod -R 755 /app

WORKDIR /app

EXPOSE 80

CMD ["/usr/bin/supervisord", "-c", "/etc/supervisord.conf"]
