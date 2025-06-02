import mmap
import numpy as np
import torch

class SharedTensor:
    def __init__(self, name, size):
        self.mem = mmap.mmap(-1, size, tagname=name)
        self.array = np.frombuffer(self.mem, dtype=np.float32)

    def to_tensor(self) -> torch.Tensor:
        return torch.from_numpy(self.array.copy()).cuda()

    def write(self, data: np.ndarray):
        assert data.dtype == np.float32
        self.mem.seek(0)
        self.mem.write(data.tobytes())
