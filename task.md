### Rust 语言特性

- [x] 将Arc修改为引用或泛型(Track1,Track2)

- [ ] 去掉PDF中的dyn (Track4)

- [ ] 宏编程，预编译 macro 生成BvhNode(Track3)

- [x] 多线程动态分发

### 其他工具的支持

- [ ] yml 或 js 加载场景 (Track5)

- [ ] benchmark (Track7)

### obj模型

- [x] obj 模型导入

- [x] 重心坐标的计算，法向量和贴图坐标在重心坐标系下的计算

- [x] mmd模型转换为obj模型，mmd姿势的截取 （胡桃、芭芭拉）
    - PMX 用 MikuMikuDance 导入,添加 VPD 姿势文件
    - 用 PMX editor 转化成obj格式，或用Blender中的MMD插件导入
    - 修改贴图错位等问题

- [x] 贴图纹理的双线性插值

- [x] obj 模型mtl 纹理参数的进一步应用

- [x] 法线贴图，形成凹凸质感

- [ ] 光照贴图（blender shading 渲染）



### book3 

- [x] book3完成，对pdf函数的支持

- [x] hittable_pdf 的改进，lightable 和lightable list


### 作品效果

- [ ] 移轴效果，散焦模糊、景深

- [x] 场景设计

- [ ] 预览效果（简单显示位置）