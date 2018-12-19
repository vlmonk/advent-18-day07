### Day 7

Ок, решаем так.

Сначала читаем файл и получаем `Vec` таких струкур

```
struct InputRecord { first: char, next: char }
```

Это значит что `first` нужно сделать перед `next`.

Дальше мы получаем HashSet всех `first` и `next` элементов.

Дальше мы делаем такую HashMap

k: char
v: HashSet<char>

Потом мы проходим по вектору (1) еще раз и заполняем hashmap.

находим элемент next и в Vec вставляем first

Подготовка закончена.

Потом ищем решение.

Проходим по всем элементам HashMap, выбираем те, у которых v пустой.
Из них ищем первый по алфовиту.

Это будет первый элемент последовательности.

Убираем его из HashMap, и прохдимся по всем v и убираем его из всех v
Повторяем пока HashMap не станет пустой (мы закончили) либо у всех элементов
Vec не пустой (что-то пошло не так)

## Задача 2

Так же получаем `HashSet` всех элементов.

Потом для каждого элемента делаем структуру:

```
struct Part {
  letter: char,
  tick: i32,
  active: boolean
}
```

Кладем это в `HashMap`

Дальше делаем ход:

- Уменьшаем tick всех активных на 1
- Смотрим все с tick == 0 и убираем их, увеличиваем кол-во воркеров.
- Смотрим все с пустыми deps и стартуем их
