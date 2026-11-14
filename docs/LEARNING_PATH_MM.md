# Zap သင်ယူရန် လမ်းကြောင်း သင်တန်းစာရင်း

ဒီလမ်းညွှန်သည် beginner မှ advanced Zap programming အထိ structured learning path တစ်ခုကို ပေးထားသည်။ အဆင့်တစ်ခုစီသည် ယခင်အဆင့်ပေါ်တွင် တည်ဆောက်ထားပြီး practical exercises များ ပါဝင်သည်။

## အကြောင်းအရာများ မျက်နှာစာ

- [Level 1: Beginner](#level-1-beginner)
- [Level 2: Intermediate](#level-2-intermediate)
- [Level 3: Advanced](#level-3-advanced)
- [Level 4: Expert](#level-4-expert)
- [Project Ideas](#project-ideas)
- [Resources](#resources)

---

## Level 1: Beginner

**လိုအပ်ချက်များ:** Basic programming knowledge (variables, functions, loops)

**ကြာချိန်:** 1-2 ပတ်

### ကိစ္စများ

1. **စတင်သင်ယူခြင်း**
   - Zap ကို install လုပ်ပါ
   - Hello World program
   - Basic syntax structure
   - Running နှင့် debugging

2. **အဓိက အယူအဆများ**
   - Variables နှင့် types (int, text, bool)
   - Arithmetic operations
   - String operations
   - Comments နှင့် code organization

3. **Control Flow**
   - `if` / `else` conditionals
   - `while` loops
   - `for` loops
   - `break` နှင့် `continue`

4. **Functions**
   - Functions သတ်မှတ်ခြင်း
   - Parameters နှင့် return values
   - Function scope
   - Basic recursion

### လေ့ကျင့်ခန်းများ

1. ၁ မှ ၁၀၀ အထိ ကိန်းများကို ပေါင်းရုံ program တစ်ခုရေးပါ
2. ကိန်းတစ်ခု စုံလား မစုံလား စစ်သော function တစ်ခုဖန်တီးပါ
3. Basic arithmetic လုပ်နိုင်သော simple calculator တစ်ခုတည်ဆောက်ပါ
4. မြှောက်လျှောက်ဇယာတစ်ခု print လုပ်သော program တစ်ခုရေးပါ

### ကိုးကားချက်
- [Language Specification](LANGUAGE_SPEC_MM.md) - Sections 1-5
- [Learn Zap](LEARN_ZAP_MM.md) - Beginner tutorials

---

## Level 2: Intermediate

**လိုအပ်ချက်များ:** Level 1 ပြီးမြောက်ရန်

**ကြာချိန်:** 2-3 ပတ်

### ကိစ္စများ

1. **Data Structures**
   - Lists (creation, indexing, operations)
   - Maps (key-value pairs)
   - Structs (custom data types)
   - Nested data structures

2. **Advanced Functions**
   - Default parameters
   - Generic functions
   - Closures နှင့် captures
   - Higher-order functions

3. **Error Handling**
   - Option type (some/none)
   - Error type
   - Pattern matching
   - Safe error propagation

4. **Modules နှင့် Imports**
   - Modules ဖန်တီးခြင်း
   - Symbols များ import လုပ်ခြင်း
   - Module scope
   - Reusable code organization

### လေ့ကျင့်ခန်းများ

1. CRUD operations ပါဝင်သော todo list application တစ်ခုတည်ဆောက်ပါ
2. Predicate တစ်ခုအပေါ် အခြေခံပြီး list ကို filter လုပ်သော function တစ်ခုဖန်တီးပါ
3. Simple hash map collision resolution တစ်ခု implement လုပ်ပါ
4. Reusable utility functions ပါဝင်သော module တစ်ခုရေးပါ

### ကိုးကားချက်
- [Language Specification](LANGUAGE_SPEC_MM.md) - Sections 6-10
- [Standard Library Index](STDLIB_INDEX_MM.md)
- [Package Guide](PACKAGE.md)

---

## Level 3: Advanced

**လိုအပ်ချက်များ:** Level 2 ပြီးမြောက်ရန်

**ကြာချိန်:** 3-4 ပတ်

### ကိစ္စများ

1. **Object-Oriented Programming**
   - Classes နှင့် methods
   - Constructors
   - Inheritance
   - Member access နှင့် mutation

2. **Type System**
   - Type annotations
   - Generic types
   - Type inference
   - Type constraints

3. **Async Programming**
   - Tasks နှင့် async/await
   - Task scheduling
   - Resource limits
   - Async best practices

4. **Database Integration**
   - SQLite operations
   - Models နှင့် migrations
   - Query builders
   - Transactions

### လေ့ကျင့်ခန်းများ

1. Web framework ဖြင့် REST API တစ်ခုတည်ဆောက်ပါ
2. SQLite database ပါဝင်သော application တစ်ခုတည်ဆောက်ပါ
3. Async file processing pipeline တစ်ခု implement လုပ်ပါ
4. Domain model အတွက် class hierarchy တစ်ခုတည်ဆောက်ပါ

### ကိုးကားချက်
- [Language Specification](LANGUAGE_SPEC_MM.md) - Sections 11-15
- [Web Framework Guide](WEB_FRAMEWORK_MM.md)
- [Database Production Guide](DATABASE_PRODUCTION_MM.md)

---

## Level 4: Expert

**လိုအပ်ချက်များ:** Level 3 ပြီးမြောက်ရန်

**ကြာချိန်:** 4-6 ပတ်

### ကိစ္စများ

1. **Compiler Internals**
   - Bootstrap stages (B0-B4)
   - Type checking algorithms
   - Bytecode generation
   - Runtime execution

2. **Performance Optimization**
   - Benchmarking
   - Memory management
   - Algorithm optimization
   - Profiling techniques

3. **System Programming**
   - Filesystem operations
   - Process management
   - Network programming
   - System integration

4. **Package Ecosystem**
   - Packages ဖန်တီးခြင်း
   - Registry သို့ publish လုပ်ခြင်း
   - Dependency management
   - Semantic versioning

### လေ့ကျင့်ခန်းများ

1. Zap compiler တွင် contribute လုပ်ပါ
2. Custom LSP language server တစ်ခုတည်ဆောက်ပါ
3. Production-ready web application တစ်ခုတည်ဆောက်ပါ
4. Registry သို့ reusable package တစ်ခု publish လုပ်ပါ

### ကိုးကားချက်
- [Bootstrap Contract](../bootstrap/contracts/BOOTSTRAP_CONTRACT_MM.md)
- [Benchmark Harness](BENCHMARK_HARNESS_MM.md)
- [Registry Auth](REGISTRY_AUTH_MM.md)

---

## Project Ideas

### Beginner Projects
- [x] Hello World - "Hello, World!" print လုပ်ပါ
- [ ] Number Guessing Game - Random number နှင့် hints
- [ ] Temperature Converter - Celsius/Fahrenheit/Kelvin
- [ ] Simple Calculator - Basic arithmetic operations
- [ ] String Reverser - Input strings ကို reverse လုပ်ပါ

### Intermediate Projects
- [ ] Todo List - File persistence ပါဝင်သော CRUD
- [ ] Contact Manager - Contacts သိမ်းဆည်းပြီး ရှာဖွေပါ
- [ ] File Analyzer - Lines, words, characters ရေတွက်ပါ
- [ ] Simple Game - Scores ပါဝင်သော number guessing
- [ ] Data Validator - JSON-like structures စစ်ဆေးပါ

### Advanced Projects
- [ ] REST API - Web framework နှင့် endpoints
- [ ] CLI Tool - Command-line application
- [ ] Database App - SQLite-backed application
- [ ] Web Scraper - Web data ယူပြီး parse လုပ်ပါ
- [ ] Async Processor - Parallel file processing

### Expert Projects
- [ ] LSP Extension - VS Code language server
- [ ] Custom Package - Reusable library
- [ ] Compiler Contribution - Language feature တစ်ခု add လုပ်ပါ
- [ ] Performance Tool - Benchmarking framework
- [ ] System Monitor - Process/file watcher

---

## Resources

### Official Documentation
- [Language Specification](LANGUAGE_SPEC_MM.md)
- [Current Status](CURRENT_STATUS_MM.md)
- [Migration Guide](MIGRATION_GUIDE_MM.md)
- [Release Notes](RELEASE_2.11.18_MM.md)

### Learning Materials
- [Learn Zap](LEARN_ZAP_MM.md)
- [Examples](../examples/)
- [Syntax Guide](SYNTAX_GUIDE.md)
- [Language Guide](LANGUAGE_GUIDE.md)

### Developer Tools
- [VS Code Extension](../vscode-extension/)
- [LSP Guide](ASYNC_LSP_MM.md)
- [Debugging Guide](RUNTIME_STATE_MM.md)
- [Testing Example](../examples/testing.zp)

### Community
- [GitHub Issues](https://github.com/hidecard/zap/issues)
- [Contributing Guide](../CONTRIBUTING.md)
- [RFC Template](../RFC_TEMPLATE.md)
- [Security Policy](../SECURITY.md)

---

## Learning Checklist

### Level 1: Beginner
- [ ] Zap install လုပ်ပြီး hello world run လုပ်ပါ
- [ ] Basic types နှင့် operations ကို နားလည်ပါ
- [ ] If/else နှင့် loop programs ရေးပါ
- [ ] Functions သတ်မှတ်ပြီး call လုပ်ပါ
- [ ] Beginner exercises ၃ ခု ပြီးမြောက်ပါ

### Level 2: Intermediate
- [ ] Lists နှင့် maps ဖြင့် အလုပ်လုပ်ပါ
- [ ] Default parameters နှင့် generics သုံးပါ
- [ ] Option/Error types ဖြင့် errors ကိုပိုင်းပါ
- [ ] Modules ဖန်တီးပြီး import လုပ်ပါ
- [ ] Intermediate exercises ၃ ခု ပြီးမြောက်ပါ

### Level 3: Advanced
- [ ] Classes နှင့် methods သတ်မှတ်ပါ
- [ ] Type annotations နှင့် generics သုံးပါ
- [ ] Tasks ဖြင့် async code ရေးပါ
- [ ] SQLite database နှင့် integrate လုပ်ပါ
- [ ] Advanced exercises ၃ ခု ပြီးမြောက်ပါ

### Level 4: Expert
- [ ] Bootstrap stages ကို နားလည်ပါ
- [ ] Code ကို profile လုပ်ပြီး optimize လုပ်ပါ
- [ ] System APIs ဖြင့် အလုပ်လုပ်ပါ
- [ ] Packages ဖန်တီးပြီး publish လုပ်ပါ
- [ ] Expert exercises ၃ ခု ပြီးမြောက်ပါ

---

## Next Steps

အဆင့်အားလုံး ပြီးမြောက်ပြီးနောက်:

1. **Zap တွင် Contribute လုပ်ပါ** - Bug တစ်ခု fix လုပ်ပါသို့မဟုတ် feature တစ်ခု add လုပ်ပြီး pull request ဖွင့်ပါ
2. **Real Project တစ်ခုတည်ဆောက်ပါ** - Production-ready application တစ်ခုဖန်တီးပါ
3. **သင့်အသိကို မျှဝေပါ** - Tutorials ရေးပါသို့မဟုတ် အခြားသူများကို သင်ယူရာတွင် ကူညီပါ
4. **Updated နေပါ** - [Roadmap](ROADMAP_MM.md) နှင့် changelog ကို လိုက်နာပါ

---

## Tips for Success

1. **နေ့စဉ် Practice လုပ်ပါ** - တစ်နေ့လျှင် တစ်ခုစီ code ရေးပါ (၁၅ မိနစ်လောက်ဖြစ်ဖွယ်)
2. **Source Code ကို ဖတ်ပါ** - Examples နှင့် compiler implementation ကို လေ့လာပါ
3. **Projects တည်ဆောက်ပါ** - Knowledge ကို real problems များတွင် အသုံးချပါ
4. **မေးမြန်းပါ** - Stuck ဖြစ်ရင် GitHub issues ကို သုံးပါ
5. **အခြားသူများကို သင်ယူပေးပါ** - Concepts များကို ရှင်းပြခြင်းသည် learning ကို ခိုင်မားစေသည်

---

## Getting Help

Stuck ဖြစ်ရင်:

- [Language Specification](LANGUAGE_SPEC_MM.md) ကို စစ်ဆေးပါ
- [Examples](../examples/) ကို စစ်ဆေးပါ
- [GitHub Issues](https://github.com/hidecard/zap/issues) တွင် ရှာပါ
- သင့်မေးခွန်းဖြင့် အသစ် issue တစ်ခုဖွင့်ပါ
