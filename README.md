# Rust Latam: procedural macros workshop

# Rust 拉丁美州：过程宏工作坊

> [Syn 过程宏项目 ReadMe 链接](https://github.com/dtolnay/proc-macro-workshop)

*This repo contains a selection of projects designed to learn to write Rust
procedural macros &mdash; Rust code that generates Rust code.*

*这个repo包含了一些旨在学习编写 Rust 的项目过程宏;生成 Rust 代码的 Rust 代码*

*Each of these projects is drawn closely from a compelling real use case. Out of
the 5 projects here, 3 are macros that I have personally implemented in
industrial codebases for work, and the other 2 exist as libraries on crates.io
by other authors.*

*这些项目中的都来自的真实项目用例。共有5个项目，其中3个是我自己基于工作中代码实现的，另外两个是其它作者实现的作为库存在于create.io上。*

<br>

## Contents

- [Rust Latam: procedural macros workshop](#rust-latam-procedural-macros-workshop)
- [Rust 拉丁美州：过程宏工作坊](#rust-拉丁美州过程宏工作坊)
  - [Contents](#contents)
  - [Suggested prerequisites](#suggested-prerequisites)
  - [先决条件](#先决条件)
  - [Projects](#projects)
  - [项目](#项目)
    - [Derive macro: `derive(Builder)`](#derive-macro-derivebuilder)
    - [派生宏： `derive(Builder)`](#派生宏-derivebuilder)
    - [Derive macro: `derive(CustomDebug)`](#derive-macro-derivecustomdebug)
    - [派生宏：`derive(CustomDebug)`](#派生宏derivecustomdebug)
    - [Function-like macro: `seq!`](#function-like-macro-seq)
    - [类函数宏：`seq!`](#类函数宏seq)
    - [Attribute macro: `#[sorted]`](#attribute-macro-sorted)
    - [属性宏: `#[sorted]`](#属性宏-sorted)
    - [Attribute macro: `#[bitfield]`](#attribute-macro-bitfield)
    - [属性宏：`#[bitfield]`](#属性宏bitfield)
    - [Project recommendations](#project-recommendations)
    - [项目建议](#项目建议)
  - [Test harness](#test-harness)
  - [测试用例](#测试用例)
  - [Workflow](#workflow)
  - [工作流](#工作流)
  - [Debugging tips](#debugging-tips)
  - [调试建议](#调试建议)

<br>

## Suggested prerequisites

## 先决条件

This workshop covers attribute macros, derive macros, and function-like
procedural macros.

这个项目涉及属性宏、派生宏和类函数过程宏。

Be aware that the content of the workshop and the explanations in this repo will
assume a working understanding of structs, enums, traits, trait impls, generic
parameters, and trait bounds. You are welcome to dive into the workshop with any
level of experience with Rust, but you may find that these basics are far easier
to learn for the first time outside of the context of macros.

请注意，本项目内容和文档中的解释假设你已经理解结构体、枚举、特型、特型 impls、泛型参数等知识。
任何Rust经验层级的开发都可以深入到本项目，但你会发现相比宏内容这些基础知识要简单的多。

<br>

## Projects

## 项目

Here is an introduction to each of the projects. At the bottom, I give
recommendations for what order to tackle them based on your interests. Note that
each of these projects goes into more depth than what is described in the
introduction here.

下面是对每个项目的介绍。下最下面，我会根据你的兴趣为你提供一些解决问题的顺序建议。请注意,
每个项目实际功能都比这里的描述更深入一些。

### Derive macro: `derive(Builder)`

### 派生宏： `derive(Builder)`

This macro generates the boilerplate code involved in implementing the [builder
pattern] in Rust. Builders are a mechanism for instantiating structs, especially
structs with many fields, and especially if many of those fields are optional or
the set of fields may need to grow backward compatibly over time.

该宏生成实现[构建器模式]所涉及的样板代码模式。构造器是一种实例化结构体的机制具有许多字段的结构，
特别是如果其中许多字段是可选的或字段集可能需要随着时间的推移向后兼容地增长。

[builder pattern]: https://en.wikipedia.org/wiki/Builder_pattern

There are a few different possibilities for expressing builders in Rust. Unless
you have a strong pre-existing preference, to keep things simple for this
project I would recommend following the example of the standard library's
[`std::process::Command`] builder in which the setter methods each receive and
return `&mut self` to allow chained method calls.

Rust中构建器模式有几种不同的实现方式，除非你有特殊的癖好，为了这个项目的简洁，我建议遵循标准库的
`std::process:Command`构建器，基中setter方法逐个接收和返回 `&mut self` 以方便链式调用。

[`std::process::Command`]: https://doc.rust-lang.org/std/process/struct.Command.html

Callers will invoke the macro as follows.

调用者会按如下方式使用宏。

```rust
use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .arg("build".to_owned())
        .arg("--release".to_owned())
        .build()
        .unwrap();

    assert_eq!(command.executable, "cargo");
}
```

This project covers:

本项目涉及：

- traversing syntax trees;
- 遍历语法树
- constructing output source code;
- 构造输出源代码
- processing helper attributes to customize the generated code.
- 处理帮助器属性来自定义生成的代码。

*Project skeleton is located under the <kbd>builder</kbd> directory.*

*项目结构位于 <kbd>builder</dbd> 目录。*

### Derive macro: `derive(CustomDebug)`

### 派生宏：`derive(CustomDebug)`

This macro implements a derive for the standard library [`std::fmt::Debug`]
trait that is more customizable than the similar `Debug` derive macro exposed by
the standard library.

该宏实现了标准库的 `std::fmt::Debug` 特型，但相比标准库的 `Debug` 派生宏更灵活。

[`std::fmt::Debug`]: https://doc.rust-lang.org/std/fmt/trait.Debug.html

In particular, we'd like to be able to select the formatting used for individual
struct fields by providing a format string in the style expected by Rust string
formatting macros like `format!` and `println!`.

特别是，我们希望能够为每个字段指定像标准库格式化宏 `formt!` 和 `println!` 一样的格式化字段串。

```rust
use derive_debug::CustomDebug;

#[derive(CustomDebug)]
pub struct Field {
    name: String,
    #[debug = "0b{:08b}"]
    bitmask: u8,
}
```

Here, one possible instance of the struct above might be printed by its
generated `Debug` impl like this:

这里，上面结构体的某个实例会使用它的 `Debug` 实现打印如下信息：

```console
Field { name: "st0", bitmask: 0b00011100 }
```

This project covers:

本项目涉及：

- traversing syntax trees;
- 遍历语法树
- constructing output source code;
- 构造输出源代码
- processing helper attributes;
- 处理帮助器属性
- dealing with lifetime parameters and type parameters;
- 处理生命周期参数和类型参数
- inferring trait bounds on generic parameters of trait impls;
- 根据特型实现（impls）中的泛型参数推断特型约束
- limitations of derive's ability to emit universally correct trait bounds.
- 根据特型能力的局限性派生出通用正确的特型约束

*Project skeleton is located under the <kbd>debug</kbd> directory.*

*项目结构位于 <kbd>debug</kbd> 目录*

### Function-like macro: `seq!`

### 类函数宏：`seq!`

This macro provides a syntax for stamping out sequentially indexed copies of an
arbitrary chunk of code.

该宏提供了一种语法，用于消除按顺序索引的重复性的任意代码块。

For example our application may require an enum with sequentially numbered
variants like `Cpu0` `Cpu1` `Cpu2` ... `Cpu511`. But note that the same `seq!`
macro should work for any sort of compile-time loop; there is nothing specific
to emitting enum variants. A different caller might use it for generating an
expression like `tuple.0 + tuple.1 + ... + tuple.511`.

例如我们的应用可能需要一个顺序编号的枚举如：`Cpu0` `Cpu1` `Cpu2` ... `Cpu511`。
但请注意同样的 `seq!` 宏应该适用于任何类型的编译时循环，生成枚举变量没什么特殊的。
其它调用者可能用他生成表达式：`tuple.0 + tuple1 + ... + tuple.511`。

```rust
use seq::seq;

seq!(N in 0..512 {
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum Processor {
        #(
            Cpu~N,
        )*
    }
});

fn main() {
    let cpu = Processor::Cpu8;

    assert_eq!(cpu as u8, 8);
    assert_eq!(cpu, Processor::Cpu8);
}
```

This project covers:

该项目涉及：

- parsing custom syntax;
- 解析自定义语法
- low-level representation of token streams;
- 记法流的低级表示
- constructing output source code.
- 构建生成源代码

*Project skeleton is located under the <kbd>seq</kbd> directory.*

*该项目位于 <kdb>seq</kdb> 目录*

### Attribute macro: `#[sorted]`

### 属性宏: `#[sorted]`

A macro for when your coworkers (or you yourself) cannot seem to keep enum
variants in sorted order when adding variants or refactoring. The macro will
detect unsorted variants at compile time and emit an error pointing out which
variants are out of order.

当你给枚举体添加成员或改造时枚举成员没有按顺序存放，该宏会在编译期检测出未排序的成员，
并报错指出哪个成员顺序有误。


```rust
#[sorted]
#[derive(Debug)]
pub enum Error {
    BlockSignal(signal::Error),
    CreateCrasClient(libcras::Error),
    CreateEventFd(sys_util::Error),
    CreateSignalFd(sys_util::SignalFdError),
    CreateSocket(io::Error),
    DetectImageType(qcow::Error),
    DeviceJail(io_jail::Error),
    NetDeviceNew(virtio::NetError),
    SpawnVcpu(io::Error),
}
```

This project covers:

该项目涉及：

- compile-time error reporting;
- 编译器错误报告
- application of visitor pattern to traverse a syntax tree;
- 使用访问者模式遍历语法树
- limitations of the currently stable macro API and some ways to work around
  them.
- 当前稳定的宏API的局限和一些相关解决办法。

*Project skeleton is located under the <kbd>sorted</kbd> directory.*

*该项目位于 <kbd>sorted</kbd> 目录*

### Attribute macro: `#[bitfield]`

### 属性宏：`#[bitfield]`

This macro provides a mechanism for defining structs in a packed binary
representation with access to ranges of bits, similar to the language-level
support for [bit fields in C].

该宏提供了一种在打包的二进制文件中定义结构体的机制，可以访问位范围的表示。类似于语言级别支持[位字段在C]

[bit fields in C]: https://en.cppreference.com/w/cpp/language/bit_field

The macro will conceptualize one of these structs as a sequence of bits 0..N.
The bits are grouped into fields in the order specified by a struct written by
the caller. The `#[bitfield]` attribute rewrites the caller's struct into a
private byte array representation with public getter and setter methods for each
field.

该宏将这些结构体概念化为0到N的位序列。这些位按照结构体指定的顺序分组到字段中。
`#[bitfield]` 属性将调用者的结构体重写为私有字节数组表示，每个字段都有公开的getter和setter方法。

The total number of bits N is required to be a multiple of 8 (this will be
checked at compile time).

总比特数N需要是8的倍数（这将在编译时检查）。

For example, the following invocation builds a struct with a total size of 32
bits or 4 bytes. It places field `a` in the least significant bit of the first
byte, field `b` in the next three least significant bits, field `c` in the
remaining four most significant bits of the first byte, and field `d` spanning
the next three bytes.

例如下面的调用构建了一个总大小为32位4比特的结构体。它将字段`a`放在第一个字段的最低有效位字节，
字段`b`在后面三个最低有效位，字段`c`在第一个字节剩余的四个最高有效位，以及字段`d`的跨度接下来的三个字节

```rust
use bitfield::*;

#[bitfield]
pub struct MyFourBytes {
    a: B1,
    b: B3,
    c: B4,
    d: B24,
}
```

```text
                               least significant bit of third byte
                                 ┊           most significant
                                 ┊             ┊
                                 ┊             ┊
║  first byte   ║  second byte  ║  third byte   ║  fourth byte  ║
╟───────────────╫───────────────╫───────────────╫───────────────╢
║▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒║
╟─╫─────╫───────╫───────────────────────────────────────────────╢
║a║  b  ║   c   ║                       d                       ║
                 ┊                                             ┊
                 ┊                                             ┊
               least significant bit of d         most significant
```

The code emitted by the `#[bitfield]` macro for this struct would be as follows.
Note that the field getters and setters use whichever of `u8`, `u16`, `u32`,
`u64` is the smallest while being at least as large as the number of bits in
the field.

`#[bitfield]` 宏对这个结构体生成的代码如下，请注意字段的getters和setters使用 `u8`, `u16`, `u32`,
`u64` 是最小的，至少与字段的位数一样大。

```rust
impl MyFourBytes {
    // Initializes all fields to 0.
    pub fn new() -> Self;

    // Field getters and setters:
    pub fn get_a(&self) -> u8;
    pub fn set_a(&mut self, val: u8);
    pub fn get_b(&self) -> u8;
    pub fn set_b(&mut self, val: u8);
    pub fn get_c(&self) -> u8;
    pub fn set_c(&mut self, val: u8);
    pub fn get_d(&self) -> u32;
    pub fn set_d(&mut self, val: u32);
}
```

This project covers:

该项目涉及：

- traversing syntax trees;
- 遍历语法树;
- processing helper attributes;
- 处理帮助属性;
- constructing output source code;
- 构建输出的源代码;
- interacting with traits and structs other than from the standard library;
- 与标准库之外的特型和结构体交互;
- techniques for compile-time assertions that require type information, by
  leveraging the trait system in interesting ways from generated code;
- 用于需要类型信息的编译时断言技术，从生成的代码中以有趣的方式利用特型系统;
- tricky code.
- 棘手的代码。

*Project skeleton is located under the <kbd>bitfield</kbd> directory.*

*该项目拉于 <kbd>bitfield</kbd> 目录*

### Project recommendations

### 项目建议

If this is your first time working with procedural macros, I would recommend
starting with the `derive(Builder)` project. This will get you comfortable with
traversing syntax trees and constructing output source code. These are the two
fundamental components of a procedural macro.

如果你是过程性宏新手，建议你从 `derive(Builder)` 项目开始。该项目会让你适应语法树遍历和构建输出源代码。
这些是过程宏的2个基础。

After that, it would be equally reasonable to jump to any of
`derive(CustomDebug)`, `seq!`, or `#[sorted]`.

随后这三个项目可以随意选中一个查看。

- Go for `derive(CustomDebug)` if you are interested in exploring how macros
  manipulate trait bounds, which is one of the most complicated aspects of
  code generation in Rust involving generic code like [Serde]. This project
  provides an approachable introduction to trait bounds and digs into many of
  the challenging aspects.

- 选择 `derive(CustomDebug)`, 如果你想了解宏如何操作特型约束，这是Rust中利用泛型生成代码最复杂的一面，像 [Serde] 一样。这个项目提供了一个平易近人的特型约束介绍，并深入研究了许多具有挑战性的方面

- Go for `seq!` if you are interested in parsing a custom input syntax yourself.
  The other projects will all mostly rely on parsers that have already been
  written and distributed as a library, since their input is ordinary Rust
  syntax.

- 选择 `seq!`，如果你想了解解析自定义语法。其他项目都将主要依赖于已经存在的解析器并已作为库分发，因为它们的输入是普通的Rust语法

- Go for `#[sorted]` if you are interested in generating diagnostics (custom
  errors) via a macro. Part of this project also covers a different way of
  processing input syntax trees; the other projects will do most things through
  `if let`. The visitor approach is better suited to certain types of macros
  involving statements or expressions as we'll see here when checking that
  `match` arms are sorted.

- 选择 `#[sorted]`， 如果你想了解通过宏生成诊断信息（自定义错误）。这个项目还涵盖了不同的方式处理输入语法树;其他项目大部分情况会使用 `if let`。 访问者方法更适合某些类型的宏包括语句或表达式，就像我们在这里检查时看到的那样 `match` 的手臂是排序的。

[Serde]: https://serde.rs/

I would recommend starting on `#[bitfield]` only after you feel you have a
strong grasp on at least two of the other projects. Note that completing the
full intended design will involve writing at least one of all three types of
procedural macros and substantially more code than the other projects.

我建议只有在你至少掌握最少2个项目时才开始 `#[bitfield]`。请注意，完成完整的预期设计将包括编写所有三种类型中的至少一种
过程宏和比其他项目多得多的代码。

<br>

## Test harness

## 测试用例

Testing macros thoroughly tends to be tricky. Rust and Cargo have a built-in
testing framework via `cargo test` which can work for testing the success cases,
but we also really care that our macros produce good error message when they
detect a problem at compile time; Cargo isn't able to say that failing to
compile is considered a success, and isn't able to compare that the error
message produced by the compiler is exactly what we expect.

彻底测试宏往往很棘手。Rust和Cargo有一个内置的 `cargo test` 测试框架，可用于测试成功案例;
但是，我们也很关心宏在编译时检测到一个问题问题是否能产生良好的错误消息;Cargo不能说错误编译被认为是成功的，
并且不能比对编译器错误正是我们所期望的。

The project skeletons in this repository use an alternative test harness called
[trybuild].

该项目使用了一个可选的测试工具：[trybuild]

[trybuild]: https://github.com/dtolnay/trybuild

<p align="center">
<a href="#test-harness">
<img src="https://user-images.githubusercontent.com/1940490/55197640-eb390080-5191-11e9-8c1f-1183935c0c26.png" width="600">
</a>
</p>

The test harness is geared toward iterating on the implementation of a
procedural macro, observing the errors emitted by failed executions of the
macro, and testing that those errors are as expected.

测试工具是为了迭代过程宏的实现，观察由于宏执行失败而发出的错误，并测试这些错误是否如预期的那样。

<br>

## Workflow

## 工作流

Every project has a test suite already written under its <kbd>tests</kbd>
directory. (But feel free to add more tests, remove tests for functionality you
don't want to implement, or modify tests as you see fit to align with your
implementation.)

每个项目在tests目录都有已实现的测试用例。（但是可以随意添加测试，删除不想实现的功能测试，修改测试以适应你的实现）

Run `cargo test` inside any of the 5 top-level project directories to run the
test suite for that project.

在5个顶级项目根目录中运行 `Cargo test` 来执行测试用例。

Initially every projects starts with all of its tests disabled. Open up the
project's *tests/progress.rs* file and enable tests one at a time as you work
through the implementation. **The test files (for example *tests/01-parse.rs*)
each contain a comment explaining what functionality is tested and giving some
tips for how to implement it.** I recommend working through tests in numbered
order, each time enabling one more test and getting it passing before moving on.

默认每个项目的测试用例都是禁用的。在你实现的过程中打开项目的 *tests/progress.rs* 文件并一次只启用一个测试。
**每个测试文件（如 *tests/01-parse.rs*）都有注释解释测试的功能并提示如何实现它。** 我建议按数字编号完成测试，
每次启动一个测试，并使其通过，然后再继续。

Tests come in two flavors: tests that should compile+run successfully, and tests
that should fail to compile with a specific error message.

测试有两种类型：应用成功编译+运行的测试，以及编译失败并显示特定的错误信息。

If a test should compile and run successfully, but fails, the test runner will
surface the compiler error or runtime error output.

如果测试应用成功编译并运行，但失败时，测试运行程序将显示编译器错误或运行时错误。

<p align="center">
<a href="#workflow">
<img src="https://user-images.githubusercontent.com/1940490/55197637-eb390080-5191-11e9-9197-5832071639ea.png" width="600">
</a>
</p>

For tests that should fail to compile, we compare the compilation output against
a file of expected errors for that test. If those errors match, the test is
considered to pass. If they do not match, the test runner will surface the
expected and actual output.

对于应该无法编译的测试，我们将比对编译输出与该测试的预期错误文件。如果这些错误匹配，
则测试是被认为通过的，如果它们不匹配，则测试运行器将显示预期和实际产出。

Expected output goes in a file with the same name as the test except with an
extension of _*.stderr_ instead of _*.rs_.

期望的输出放在与测试同名的文件中，只是扩展名是 _*.stderr_ 而不是 _*.rs_。

<p align="center">
<a href="#workflow">
<img src="https://user-images.githubusercontent.com/1940490/55197639-eb390080-5191-11e9-9c8f-a47cab89652d.png" width="600">
</a>
</p>

If there is no _*.stderr_ file for a test that is supposed to fail to compile,
the test runner will save the compiler's output into a directory called
<kbd>wip</kbd> adjacent to the <kbd>tests</kbd> directory. So the way to update
the "expected" output is to delete the existing _*.stderr_ file, run the tests
again so that the output is written to *wip*, and then move the new output from
*wip* to *tests*.

如果用于编译失败的测试没有生成 _*.stderr_ 文件，测试运行程序将会保存编译器输出到紧邻 <kbd>tests</kbd>
的 <kbd>wip</kbd> 目录。所以更新"期望"的输入的方法是删除存在的 _.*.stderr_ 文件，再运行一次测试会
把输出写入 *wip*，再移动把新生成的输出从 *wip* 移到 *tests* 。

<p align="center">
<a href="#workflow">
<img src="https://user-images.githubusercontent.com/1940490/55197642-ebd19700-5191-11e9-8f00-2d7c5f4be1a9.png" width="600">
</a>
</p>

<br>

## Debugging tips

## 调试建议

To look at what code a macro is expanding into, install the [cargo expand] Cargo
subcommand and then run `cargo expand` in the repository root (outside of any of
the project directories) to expand the main.rs file in that directory. You can
copy any of the test cases into this main.rs and tweak it as you iterate on the
macro.

要查看宏展开的代码，安装 Cargo 子命令 [cargo expand] 然后在根目录（在任何项目目录之外）执行 `cargo expand` 
来扩展目录的 main.rs 文件。你可以将任何测试用例复制到这个 main.rs 中。并在迭代时进行调整宏。

[cargo expand]: https://github.com/dtolnay/cargo-expand

If a macro is emitting syntactically invalid code (not just code that fails
type-checking) then cargo expand will not be able to show it. Instead have the
macro print its generated TokenStream to stderr before returning the tokens.

如果宏发出语法上无效的代码(不仅仅是失败代码的类型检查)则货 cargo  expand 将无法显示它。
取而代之的是宏在返回令牌之前将其生成的 TokenStream 打印到标签错误输出。

```rust
eprintln!("TOKENS: {}", tokens);
```

Then a `cargo check` in the repository root (if you are iterating using main.rs)
or `cargo test` in the corresponding project directory will display this output
during macro expansion.

然后在根目录下运行 `cargo check` （如何你正在使用 main.rs），或在宏扩展期间在对应项目目录执行 `cargo test`
也会显示此输出

Stderr is also a helpful way to see the structure of the syntax tree that gets
parsed from the input of the macro.

Stderr 标准输出也是查看从宏的输入解析得到的语法树结构的有用方法

```rust
eprintln!("INPUT: {:#?}", syntax_tree);
```

Note that in order for Syn's syntax tree types to provide Debug impls, you will
need to set `features = ["extra-traits"]` on the dependency on Syn. This is
because adding hundreds of Debug impls adds an appreciable amount of compile
time to Syn, and we really only need this enabled while doing development on a
macro rather than when the finished macro is published to users.

请注意，为了让 Syn 的语法树类型提供 Debug 实现，您需要需要在 Syn 依赖上设置 `features = ["extra-traits"]`
因为添加数百个 Debug 实现会增加相当数量的编译时间，主要我们只是在开发时需要启用这个功能
而不是在完成的宏发布给用户时。

