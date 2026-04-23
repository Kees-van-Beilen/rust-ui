// SPDX-FileCopyrightText: 2026  The rust-ui maintainers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

import android.text.TextWatcher;
import android.text.Editable;
import java.lang.CharSequence;

public class RustUITextWatcher implements TextWatcher {
    private long rawFunctionBox;
    /// consume the rawFunctionBox making future onClick calls panic
    public native void destroy();
    /// the implementor
    @Override
    public native void afterTextChanged(Editable editable);
    @Override 
    public void beforeTextChanged(CharSequence s, int start, int before, int count) {

    }
    @Override 
    public void onTextChanged(CharSequence s, int start, int before, int count) {
        
    }
}
