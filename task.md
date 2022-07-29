# Ray-Tracing-in-One-Weekend
ACM class PPCA Program

### Monte Carlo 积分

考虑光的散射公式（对`direction`的积分）

$$
Color = \int  A \cdot \mathrm{scatter\_ pdf}(direction) \cdot \rm{color}(direction)
$$

`direction`可以看成是随机生成的，在使用`random_unit_vec()`函数生成`direction`的时候，`direction`并不均匀随机，其满足了$\cos(\phi)$的分布，于是在用MC方法估计积分值的时候需要除以立体角的概率密度函数$\rm{pdf} = \frac{\cos(\phi)}{\pi}$（除以$\pi$是为了保证积分值为1）。

$$
Color = \frac{A \cdot \mathrm{scatter\_ pdf}(direction) \cdot \rm{color}(direction)}{\rm{pdf}}
$$

不难发现$\rm{pdf} = \rm{scatter\_ pdf}$ ,其实第一本书就是在这里隐式地用了这个方法。

如果`direction`换成均匀随机，换成均匀的概率密度函数即可。

### obj 模型的导入以及贴图

- 添加`Lightable triat`,将`hittable_pdf`改进了具有`LightableList`
- 使用`tobj`库实现了`obj`模型的导入。
- 使用`cramer rule`求直线和三角形的交，并且按照**重心坐标系**实现了`hit point`的法向量，贴图的纹理坐标`texcoords`

## Tasks

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