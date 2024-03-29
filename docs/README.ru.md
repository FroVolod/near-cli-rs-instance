near-cli
--------
near-cli – это утилита командной строки для работы с блокчейном Near Protocol.

## README.md

* en [English](README.en.md)

## Оглавление

* [Применение](#применение)
* [Инструкция](#инструкция)
* [Установка](#установка)
* [Сборка](#сборка)

## Применение

Вцелом новичку трудно сразу разобраться как устроены команды.  
Например, для осуществления перевода токенов необходимо набрать в терминале такую команду:

```txt
./near-cli tokens \
    'volodymyr.testnet' \
    send-near \
    'fro_volod.testnet' \
    '1 NEAR' \
    network testnet \
    sign-with-keychain \
    send
```

Это полная команда. Результат ее работы будет такой:

```txt
Successful transaction
<volodymyr.testnet> has transferred 1 NEAR to <fro_volod.testnet> successfully.
Transaction ID: G4t6Sgz2FjnNpruYjPP1ZJAKfRmBffVaqmj8Nup2TaAg
To see the transaction in the transaction explorer, please open this url in your browser:
https://explorer.testnet.near.org/transactions/G4t6Sgz2FjnNpruYjPP1ZJAKfRmBffVaqmj8Nup2TaAg
```

Набирая эту или другую команду вручную, Вы можете допустить ошибку, либо забыть последовательность набора команды.  
Не проблема, `--help` подскажет как правильно выстроить команду.  
Однако, используя near-cli, Вы __в любом месте набора команды__ можете нажать Enter и интерактивный режим программы продолжит работу по составлению команды с того места, где Вы закончили вводить необходимые параметры.

<details><summary><i>Демонстрация работы утилиты с частично набранной командой</i></summary>
<a href="https://asciinema.org/a/AfxLN1QtJi1z1qXuowTj2nDw2?autoplay=1&t=1&speed=2">
    <img src="https://asciinema.org/a/AfxLN1QtJi1z1qXuowTj2nDw2.png" width="836"/>
</a>
</details>


## Инструкция

Подробная инструкция доступна в файле [GUIDE.ru.md](GUIDE.ru.md).

## Установка

На данном этапе разработки утилиты установка программы не требуется.  
Достаточно [загрузить](https://github.com/FroVolod/near-cli/releases/) архивный файл, подходящий к Вашей операциой системе, установленной на компьютере, и разархивировать его.  
В полученном каталоге находится исполняемый файл _near-cli_, к которому прилагается [подробная инструкция для пользователя](GUIDE.ru.md).

## Сборка

near-cli написан на Rust. Поэтому необходимо
[установить Rust](https://www.rust-lang.org/) для компиляции программы.
near-cli компилируется на версии Rust 1.61.0 (stable) или новее.

Сборка near-cli:

```txt
$ git clone https://github.com/FroVolod/near-cli
$ cd near-cli
$ cargo build --release
$ ./target/release/near-cli --version
near-cli 0.1.0
```
