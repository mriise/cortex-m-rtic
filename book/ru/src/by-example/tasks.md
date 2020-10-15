# Программные задачи

RTIC обрабатывает прерывания и исключения как *аппаратные* задачи. Аппаратные
задачи могут вызываться устройством в ответ на события, такие как нажатие кнопки.
RTIC также поддерживает *программные* задачи, порождаемые программой из любого
контекста выполнения.

Программным задачам также можно назначать приоритет и диспетчеризовать из
обработчиков прерываний. RTIC требует определения свободных прерываний в блоке
`extern`, когда используются программные задачи; эти свободные прерывания будут использованы, чтобы диспетчеризовать программные задачи. Преимущество программных
задач перед аппаратными в том, что  на один обработчик прерывания можно назначить
множество задач.

Программные задачи определяются заданием функциям атрибута `task`. Чтобы было
возможно вызывать программные задачи, имя задачи нужно передать в аргументе
`spawn` контекста атрибута (`init`, `idle`, `interrupt`, etc.).

В примере ниже продемонстрированы три программных задачи, запускаемые на 2-х
разных приоритетах. Трем задачам назначены 2 обработчика прерываний.

``` rust
{{#include ../../../../examples/task.rs}}
```

``` console
$ cargo run --example task
{{#include ../../../../ci/expected/task.run}}```

## Передача сообщений

Другое преимущество программных задач - возможность передавать сообщения задачам
во время их вызова. Тип полезной нагрузки сообщения должен быть определен в
сигнатуре обработчика задачи.

Пример ниже демонстрирует три задачи, две из которых ожидают сообщения.

``` rust
{{#include ../../../../examples/message.rs}}
```

``` console
$ cargo run --example message
{{#include ../../../../ci/expected/message.run}}```

## Ёмкость

Диспетчеры задач *не* используют динамическое выделение памяти. Память
необходимая для размещения сообщений, резервируется статически. Фреймворк
зарезервирует достаточно памяти для каждого контекста, чтобы можно было вызвать
каждую задачу как минимум единожды. Это разумно по умолчанию, но
"внутреннюю" ёмкость каждой задачи можно контролировать используя аргумент
`capacity` атрибута `task`.

В примере ниже установлена ёмкость программной задачи `foo` на 4. Если ёмкость
не определена, тогда второй вызов `spawn.foo` в `UART0` вызовет ошибку.

``` rust
{{#include ../../../../examples/capacity.rs}}
```

``` console
$ cargo run --example capacity
{{#include ../../../../ci/expected/capacity.run}}```