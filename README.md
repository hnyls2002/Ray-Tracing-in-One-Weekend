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