use std::path::Path;

use libc::pid_t;
use pyo3::exceptions::{PyException, PyIOError};
use pyo3::prelude::*;
use pyo3::{create_exception, wrap_pyfunction};
use rsprocmaps::error::Error;

create_exception!(procmaps, ParseError, PyException);

/// Represents a memory map in the maps file.
#[pyclass]
struct Map {
    inner: rsprocmaps::Map,
}

#[pymethods]
impl Map {
    /// Returns the beginning address for the map.
    #[getter]
    fn begin_address(&self) -> u64 {
        self.inner.address_range.begin
    }

    /// Returns the end address for the map.
    #[getter]
    fn end_address(&self) -> u64 {
        self.inner.address_range.end
    }

    /// Returns whether this map has the 'read' bit set
    #[getter]
    fn is_readable(&self) -> bool {
        self.inner.permissions.readable
    }

    /// Returns whether this map has the 'write' bit set
    #[getter]
    fn is_writable(&self) -> bool {
        self.inner.permissions.writable
    }

    /// Returns whether this map has 'executable' bit set
    #[getter]
    fn is_executable(&self) -> bool {
        self.inner.permissions.executable
    }

    /// Returns whether this map is shared with other processes
    #[getter]
    fn is_shared(&self) -> bool {
        self.inner.permissions.shared
    }

    /// Returns whether this map is private (i.e., copy-on-write)
    #[getter]
    fn is_private(&self) -> bool {
        self.inner.permissions.private
    }

    /// Returns the offset of the source that this map begins at.
    #[getter]
    fn offset(&self) -> u64 {
        self.inner.offset
    }

    /// Returns the (major, minor) tuple for the associated device.
    #[getter]
    fn device(&self) -> (u64, u64) {
        (self.inner.device.major, self.inner.device.minor)
    }

    /// Returns the inode associated with the source and device, or 0 if
    /// no inode is associated.
    #[getter]
    fn inode(&self) -> u64 {
        self.inner.inode
    }

    /// Returns the pathname (or pseudo-path) associated with the map,
    /// or None if the map is an anonymous map.
    #[getter]
    fn pathname(&self) -> Option<String> {
        match &self.inner.pathname {
            rsprocmaps::Pathname::Stack => Some("[stack]".into()),
            rsprocmaps::Pathname::Vdso => Some("[vdso]".into()),
            rsprocmaps::Pathname::Vvar => Some("[vvar]".into()),
            rsprocmaps::Pathname::Vsyscall => Some("[vsyscall]".into()),
            rsprocmaps::Pathname::Heap => Some("[heap]".into()),
            rsprocmaps::Pathname::OtherPseudo(p) => Some(p.into()),
            rsprocmaps::Pathname::Path(p) => Some(p.into()),
            rsprocmaps::Pathname::Mmap => None,
        }
    }

    fn __contains__(&self, addr: u64) -> PyResult<bool> {
        Ok(addr >= self.inner.address_range.begin && addr < self.inner.address_range.end)
    }
}

// NOTE(ww): Trait impl stupidity.
struct ProcmapsError(Error);
impl std::convert::From<ProcmapsError> for PyErr {
    fn from(err: ProcmapsError) -> PyErr {
        match err.0 {
            Error::Io(e) => PyIOError::new_err(e),
            Error::ParseError(e) => ParseError::new_err(e.to_string()),
            Error::WidthError(e) => ParseError::new_err(e),
        }
    }
}

/// Returns the maps for the given process.
#[pyfunction]
fn from_pid(pid: pid_t) -> PyResult<Vec<Map>> {
    let mut maps = Vec::new();

    let inner_maps = rsprocmaps::from_pid(pid).map_err(ProcmapsError)?;
    for map in inner_maps {
        maps.push(Map {
            inner: map.map_err(ProcmapsError)?,
        })
    }

    Ok(maps)
}

/// Returns the maps in the given file.
#[pyfunction]
fn from_path(path: &str) -> PyResult<Vec<Map>> {
    let mut maps = Vec::new();

    let inner_maps = rsprocmaps::from_path(Path::new(path)).map_err(ProcmapsError)?;
    for map in inner_maps {
        maps.push(Map {
            inner: map.map_err(ProcmapsError)?,
        })
    }

    Ok(maps)
}

/// Returns the maps in the given string.
#[pyfunction]
fn from_str(maps_data: &str) -> PyResult<Vec<Map>> {
    let mut maps = Vec::new();

    let inner_maps = rsprocmaps::from_str(maps_data);
    for map in inner_maps {
        maps.push(Map {
            inner: map.map_err(ProcmapsError)?,
        })
    }

    Ok(maps)
}

#[pymodule]
fn procmaps(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Map>()?;
    m.add_wrapped(wrap_pyfunction!(from_pid))?;
    m.add_wrapped(wrap_pyfunction!(from_path))?;
    m.add_wrapped(wrap_pyfunction!(from_str))?;

    Ok(())
}
