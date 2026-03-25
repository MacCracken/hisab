use super::complex::Complex;
use crate::HisabError;

/// In-place Cooley-Tukey radix-2 FFT.
///
/// `data` must have a power-of-2 length. Computes the DFT in-place.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `data.len()` is not a power of two.
#[must_use = "returns an error if input length is not a power of two"]
pub fn fft(data: &mut [Complex]) -> Result<(), HisabError> {
    let n = data.len();
    if n <= 1 {
        return Ok(());
    }
    if !n.is_power_of_two() {
        return Err(HisabError::InvalidInput(
            "FFT requires power-of-2 length".into(),
        ));
    }

    // Bit-reversal permutation
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            data.swap(i, j);
        }
    }

    // Butterfly stages
    let mut len = 2;
    while len <= n {
        let half = len / 2;
        let angle = -2.0 * std::f64::consts::PI / len as f64;
        let wn = Complex::new(angle.cos(), angle.sin());

        let mut start = 0;
        while start < n {
            let mut w = Complex::new(1.0, 0.0);
            for k in 0..half {
                let u = data[start + k];
                let t = w * data[start + k + half];
                data[start + k] = u + t;
                data[start + k + half] = u - t;
                w = w * wn;
            }
            start += len;
        }
        len <<= 1;
    }
    Ok(())
}

/// In-place inverse FFT.
///
/// `data` must have a power-of-2 length.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `data.len()` is not a power of two.
#[must_use = "returns an error if input length is not a power of two"]
pub fn ifft(data: &mut [Complex]) -> Result<(), HisabError> {
    let n = data.len();
    // Conjugate, FFT, conjugate, scale
    for d in data.iter_mut() {
        *d = d.conj();
    }
    fft(data)?;
    let scale = 1.0 / n as f64;
    for d in data.iter_mut() {
        *d = d.conj() * scale;
    }
    Ok(())
}

/// In-place 2D FFT on a row-major grid of `rows × cols`.
///
/// Both `rows` and `cols` must be powers of two.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if dimensions aren't powers of two
/// or `data.len() != rows * cols`.
#[must_use = "returns an error if dimensions are invalid"]
pub fn fft_2d(data: &mut [Complex], rows: usize, cols: usize) -> Result<(), HisabError> {
    if data.len() != rows * cols {
        return Err(HisabError::InvalidInput(format!(
            "data length {} != rows*cols {}",
            data.len(),
            rows * cols
        )));
    }
    // FFT each row
    for r in 0..rows {
        let row = &mut data[r * cols..(r + 1) * cols];
        fft(row)?;
    }
    // FFT each column (extract, transform, put back)
    let mut col_buf = vec![Complex::new(0.0, 0.0); rows];
    for c in 0..cols {
        for r in 0..rows {
            col_buf[r] = data[r * cols + c];
        }
        fft(&mut col_buf)?;
        for r in 0..rows {
            data[r * cols + c] = col_buf[r];
        }
    }
    Ok(())
}

/// In-place 2D inverse FFT on a row-major grid.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if dimensions aren't powers of two
/// or `data.len() != rows * cols`.
#[must_use = "returns an error if dimensions are invalid"]
pub fn ifft_2d(data: &mut [Complex], rows: usize, cols: usize) -> Result<(), HisabError> {
    if data.len() != rows * cols {
        return Err(HisabError::InvalidInput(format!(
            "data length {} != rows*cols {}",
            data.len(),
            rows * cols
        )));
    }
    for r in 0..rows {
        let row = &mut data[r * cols..(r + 1) * cols];
        ifft(row)?;
    }
    let mut col_buf = vec![Complex::new(0.0, 0.0); rows];
    for c in 0..cols {
        for r in 0..rows {
            col_buf[r] = data[r * cols + c];
        }
        ifft(&mut col_buf)?;
        for r in 0..rows {
            data[r * cols + c] = col_buf[r];
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Discrete Sine / Cosine Transforms
// ---------------------------------------------------------------------------

/// Discrete Sine Transform Type-I (DST-I).
///
/// Computes `X[k] = Σ x[n] · sin(π·(n+1)·(k+1) / (N+1))` for `k = 0..N-1`.
///
/// Used for wall-bounded Poisson solvers where the solution vanishes at both
/// boundaries (Dirichlet conditions).
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `data` is empty.
#[must_use = "contains the DST coefficients or an error"]
pub fn dst(data: &[f64]) -> Result<Vec<f64>, HisabError> {
    let n = data.len();
    if n == 0 {
        return Err(HisabError::InvalidInput(
            "DST requires non-empty input".into(),
        ));
    }
    let np1 = (n + 1) as f64;
    let mut out = Vec::with_capacity(n);
    for k in 0..n {
        let mut sum = 0.0;
        for (i, &x) in data.iter().enumerate() {
            sum += x * (std::f64::consts::PI * (i + 1) as f64 * (k + 1) as f64 / np1).sin();
        }
        out.push(sum);
    }
    Ok(out)
}

/// Inverse Discrete Sine Transform Type-I (IDST-I).
///
/// DST-I is its own inverse up to a scale factor of `2 / (N+1)`.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `data` is empty.
#[must_use = "contains the inverse DST result or an error"]
pub fn idst(data: &[f64]) -> Result<Vec<f64>, HisabError> {
    let n = data.len();
    let mut out = dst(data)?;
    let scale = 2.0 / (n + 1) as f64;
    for v in &mut out {
        *v *= scale;
    }
    Ok(out)
}

/// Discrete Cosine Transform Type-II (DCT-II).
///
/// Computes `X[k] = Σ x[n] · cos(π·(2n+1)·k / (2N))` for `k = 0..N-1`.
///
/// Used for Neumann boundary conditions where the derivative vanishes at
/// boundaries. Also the basis of JPEG compression.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `data` is empty.
#[must_use = "contains the DCT coefficients or an error"]
pub fn dct(data: &[f64]) -> Result<Vec<f64>, HisabError> {
    let n = data.len();
    if n == 0 {
        return Err(HisabError::InvalidInput(
            "DCT requires non-empty input".into(),
        ));
    }
    let two_n = 2.0 * n as f64;
    let mut out = Vec::with_capacity(n);
    for k in 0..n {
        let mut sum = 0.0;
        for (i, &x) in data.iter().enumerate() {
            sum += x * (std::f64::consts::PI * (2.0 * i as f64 + 1.0) * k as f64 / two_n).cos();
        }
        out.push(sum);
    }
    Ok(out)
}

/// Inverse Discrete Cosine Transform (IDCT / DCT-III).
///
/// Inverts `dct()`: `x[n] = X[0]/N + (2/N)·Σ_{k=1}^{N-1} X[k]·cos(π·k·(2n+1)/(2N))`.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `data` is empty.
#[must_use = "contains the inverse DCT result or an error"]
pub fn idct(data: &[f64]) -> Result<Vec<f64>, HisabError> {
    let n = data.len();
    if n == 0 {
        return Err(HisabError::InvalidInput(
            "IDCT requires non-empty input".into(),
        ));
    }
    let two_n = 2.0 * n as f64;
    let inv_n = 1.0 / n as f64;
    let dc = data[0] * inv_n;
    let scale = 2.0 * inv_n;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let mut sum = dc;
        for (k, &x) in data.iter().enumerate().skip(1) {
            sum += scale
                * x
                * (std::f64::consts::PI * k as f64 * (2.0 * i as f64 + 1.0) / two_n).cos();
        }
        out.push(sum);
    }
    Ok(out)
}
