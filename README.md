分包说明
1. base（基础工具包）： 与业务无关的工具类与通用配置管理。
2. domain（领域包）： 分层架构中的领域层，包含了系统的各个聚合、适配器、工具类与共享的业务配置等。
3. infrastructure（基础设施包）： 分层架构中的基础设施层，包含系统配置、适配器实现、仓储实现等支撑系统运行的基础能力。
4. application（应用服务包）： 分层架构中的应用层，是整个系统的门面，增删改查业务逻辑出入口定义在此。
5. interaction（用户交互包）： 分层架构中的Api层、Controller、MqListener接口实现等外部请求入口均定义在此处。
6. bin（启动包）： 配置文件定义，日志管理定义等全局处理配置定义。

domain:
```text
├── adapter     防腐层/适配器接口定义   
│   └── model   防腐层/适配器所需的出入参实体定义，用于包装外部
├── aggregate
│   └── user    按照聚合分包，比如这里表示用户聚合
│       ├── constant  当前上下文内通用的常量
│       ├── event     当前上下文内领域事件
│       ├── model     聚合根与实体存放
│       ├── repository  聚合对应的仓储接口
│       └── service   领域服务接口
│           └── impl  领域服务实现类
└── share  领域层共享的工具与能力
    ├── enums  业务枚举
    ├── event  领域事件定义、发送、接受等接口定义
    ├── exception 业务异常共享
    └── valueobject 值对象本身只是一个结构，没有生命周期，也共享

```

infrastructure
```text
├── adapter 对应domain包下适配器接口的实现
├── config  全局性系统配置或者业务配置
├── constant 全局系统配置共享常量
├── db
│   ├── converter 领域模型与数据模型转换器
│   ├── mapper mybatis-plus的Dao层接口
│   ├── model 持久化实体，对应数据模型
│   └── repository 仓储实现类
├── event 事件发送与处理等实现
├── mq
│   └── producer mq发送实现
└── rpc 无意义，为了演示而摆在这里的，可忽略

```