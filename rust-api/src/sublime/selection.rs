use super::imports::prelude::*;
use super::Region;
/// this struct was crated because of this error
/// from sublime console
/// TypeError: 'Selection' object cannot be converted to 'PyList'
/// when trying to do this
/// let selection_area = selection_area.into_ref(py).downcast::<PyList>()?;
pub struct Selection<'a> {
    python:    Python<'a>,
    pub inner: PyObject,
}

impl<'a> Selection<'a> {
    pub fn from_raw(
        python: Python<'a>,
        object: PyObject,
    ) -> Self {
        Self {
            python,
            inner: object,
        }
    }

    pub fn get_item_usize(
        &'a self,
        index: usize,
    ) -> PyResult<usize> {
        let item = self.inner.call_method(
            self.python,
            PyString::new(self.python, "__getitem__"),
            (index,),
            None,
        )?;
        item.extract::<usize>(self.python)
    }

    pub fn to_region(&self) -> PyResult<Region> {
        let region_raw = self.inner.call_method(
            self.python,
            PyString::new(self.python, "__getitem__"),
            (0,),
            None,
        )?;

        let region = Region::from_raw(self.python, region_raw);

        Ok(region)
    }

    pub fn get_item(
        &'a self,
        index: usize,
    ) -> PyResult<PyObject> {
        let region = self.inner.call_method(
            self.python,
            PyString::new(self.python, "__getitem__"),
            (index,),
            None,
        )?;

        let tuple = region.call_method(
            self.python,
            PyString::new(self.python, "to_tuple"),
            (),
            None,
        )?;

        Ok(tuple)
    }

    // this is BUGGY
    pub fn get_item_tuple(
        &'a self,
        index: usize,
    ) -> PyResult<(usize, usize)> {
        let item = self.inner.call_method(
            self.python,
            PyString::new(self.python, "__getitem__"),
            (index,),
            None,
        )?;
        item.extract::<(usize, usize)>(self.python)
    }

    pub fn len(&self) -> PyResult<usize> {
        let length = self.inner.call_method(
            self.python,
            PyString::new(self.python, "__len__"),
            (),
            None,
        )?;
        let length = length.extract::<usize>(self.python)?;
        Ok(length)
    }

    pub fn clear(&mut self) -> PyResult<()> {
        self.inner.call_method(
            self.python,
            PyString::new(self.python, "clear"),
            (),
            None,
        )?;

        Ok(())
    }

    // pub fn add(
    //     &self,
    //     region: &PyObject,
    // ) -> PyResult<()> {
    //     self.inner.call_method(
    //         self.python,
    //         PyString::new(python, "add"),
    //         (region,),
    //         None,
    //     )?;

    //     Ok(())
    // }

    pub fn get_attr(
        &self,
        name: &str,
    ) -> PyResult<Py<PyAny>> {
        let attribute = self
            .inner
            .getattr(self.python, PyString::new(self.python, name));
        attribute
    }
}
